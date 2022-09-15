use reqwest::Client;
use anyhow::{Result, anyhow};
use clap::Parser;

/// 羊了个羊一键闯关
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 抓包获取请求头中的t
    #[clap(short, long)]
    t: String,

    /// 通关时间
    #[clap(short, long, default_value_t=120)]
    time: u32,

}

// 
async fn finish_game(token: String, rank_time: u32) -> Result<()> {
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E158 MicroMessenger/8.0.26(0x18002b36) NetType/5G Language/zh_CN";
    let url = format!("https://cat-match.easygame2021.com/sheep/v1/game/game_over?rank_score=1&rank_state=1&rank_time={}&rank_role=1&skin=1", rank_time);
    let data = Client::builder()
        .user_agent(user_agent)
        .build()?
        .get(url)
        .header("t", token)
        .header("Content-Type", "application/json")
        .header("host", "cat-match.easygame2021.com")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    if data.get("err_code").is_some() && data.get("err_code").unwrap().as_u64().unwrap() == 0 {
        Ok(())
    }else {
        Err(anyhow!("请检查配置项t."))
    }
    
}
 

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match finish_game(args.t, args.time).await {
        Ok(_) => println!("恭喜成功完成闯关..."),
        Err(e) => println!("闯关失败, {e}"),
    };
    Ok(())
}