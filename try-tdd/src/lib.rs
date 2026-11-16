/// `contents` の中から `query` を含む行をすべて返す。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TODO: テストを通す最小限の実装を書く（Green）
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 🔴 Red: 最初のテスト。まずはこれが失敗することを確認する。
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
