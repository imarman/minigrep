use clap::{Parser};

#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {

    /// 搜索的关键字
    #[clap(long, short)]
    pub keyword: String,

    /// 文件路径
    #[clap(long, short)]
    pub file_path: String,

    /// 是否忽略大小写 1:忽略 0:不忽略
    #[clap(long, short, default_value_t = 0)]
    pub ignore_case: u8,

}
