use super::prelude::*;
use crate::from_c::stat as sys_stat;
use serde::Serialize;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Stat,
        vec!["pathname", "stat"],
        vec![AV::CString(In), AV::CStruct(Out, CStructAV::sys_stat)],
        AV::Int(Out),
    );

    inp.add(
        Ident::Fstat,
        vec!["pathname", "stat"],
        vec![AV::Int(In), AV::CStruct(Out, CStructAV::sys_stat)],
        AV::Int(Out),
    );
}

#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Stat path {:?} returned {:?}", self.pathname, self.stat))]
pub struct Stat {
    #[hstrace]
    pub pathname: String,

    #[hstrace(c_struct = sys_stat)]
    pub stat: StatResult,
}

#[derive(Debug, Clone, PartialEq, FromCStruct, Serialize)]
#[hstrace(c_struct = sys_stat)]
pub struct StatResult {
    // st_dev=makedev(0xfd, 0),
    #[hstrace]
    pub st_ino: usize,

    // st_mode=S_IFDIR|S_ISVTX|0777,
    #[hstrace]
    pub st_nlink: usize,

    #[hstrace]
    pub st_uid: usize,

    #[hstrace]
    pub st_gid: usize,

    #[hstrace]
    pub st_blksize: usize,

    #[hstrace]
    pub st_blocks: usize,

    #[hstrace]
    pub st_size: usize,
    // st_atime=1582152368 /* 2020-02-20T00:46:08.817344421+0200 */,
    // st_atime_nsec=817344421,
    // st_mtime=1582364642 /* 2020-02-22T11:44:02.282182811+0200 */,
    // st_mtime_nsec=282182811,
    // st_ctime=1582364642 /* 2020-02-22T11:44:02.282182811+0200 */,
    // st_ctime_nsec=282182811
}
