use crossbeam_channel::Receiver;
use num_traits::FromPrimitive;

use crate::syscall::Ident;
use crate::trace_grouper;
use crate::TraceOutput;

pub struct HTraceIterator {
    matchers: Vec<trace_grouper::Match>,
    receiver: Receiver<TraceOutput>,
}

impl HTraceIterator {
    pub fn new(receiver: Receiver<TraceOutput>) -> Self {
        let matchers = trace_grouper::get_matchers();

        HTraceIterator { matchers, receiver }
    }
}

impl Iterator for HTraceIterator {
    type Item = trace_grouper::GroupedIdent;

    fn next(&mut self) -> Option<Self::Item> {
        let mut do_return = None;

        loop {
            let next = match self.receiver.recv() {
                Ok(r) => r,
                Err(_e) => return None,
            };

            let call: Ident = FromPrimitive::from_usize(next.nr).unwrap();

            for (matcher_num, matcher) in &mut self.matchers.iter_mut().enumerate() {
                if matcher.match_state > 0 {
                    let item = &mut matcher.items[matcher.match_state - 1];
                    if item.mode == trace_grouper::MatchMode::Multiple {
                        // were in multiple-match mode, check if we find one from there
                        if let Some(_) = item.calls.iter_mut().find(|c| call == c.call) {
                            // found one, break
                            item.match_count += 1;
                            matcher.grouped.as_mut().unwrap().calls.push(next);
                            break;
                        }
                    }
                }

                let items_len = matcher.items.len();
                let mut item = &mut matcher.items[matcher.match_state];
                // println!("\tstate: {} -> find from {:?}!", matcher.match_state, item);

                if let Some(found_item_call) = item.calls.iter().find(|c| call == c.call) {
                    matcher.match_state += 1;
                    item.match_count += 1;

                    if items_len == matcher.match_state {
                        // println!("\t\tReached last one!");
                        matcher.grouped.as_mut().unwrap().calls.push(next);
                        do_return = Some(matcher_num);
                        break;
                    } else {
                        // println!("\t\tincrement state");
                        if let Some(_store_arg) = &found_item_call.store_to {
                            // FIXME check inp len?
                            /*
                            matcher
                                .store
                                .insert(store_arg.to_hashmap, next.inp[store_arg.from_arg].clone());
                                 */ // FIXME doesn't work
                        }

                        matcher.grouped.as_mut().unwrap().calls.push(next);
                        break;
                    }
                } else {
                    // no matches, send just a single one
                    return Some(trace_grouper::GroupedIdent::from_call(next));
                }
            }

            if let Some(matcher_num) = do_return {
                let to_return = Some(self.matchers[matcher_num].grouped.take().unwrap());
                self.matchers[matcher_num].grouped = Some(trace_grouper::GroupedIdent::new());
                self.matchers[matcher_num].match_state = 0;

                return to_return;
            }
        }
    }
}
