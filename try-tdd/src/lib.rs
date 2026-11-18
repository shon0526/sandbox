/// `contents` の中から `query` を含む行をすべて返す。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let search_result: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    search_result
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

    // どの行も一致しない場合
    #[test]
    fn empty_result() {
        let query = "none";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let output: Vec<&str> = vec![];
        assert_eq!(output, search(query, contents));
    }

    // クエリが複数単語かつ複数行に対して一致する場合
    #[test]
    fn multi_query() {
        let query = "many money";
        let contents = "\
I have many money.
I want to eat launch.
I want to many money.
That's all.";
        assert_eq!(
            vec!["I have many money.", "I want to many money."],
            search(query, contents)
        );
    }

    //空白の場合
    #[test]
    fn kuhaku_result() {
        let query = " ";
        let contents = "\
I have many money.
I want to eat launch.
I want to many money.
That's all.";
        assert_eq!(
            vec![
                "I have many money.",
                "I want to eat launch.",
                "I want to many money.",
                "That's all."
            ],
            search(query, contents)
        );
    }

    // queryに同じ単語が２回出てきたとき
    #[test]
    fn twice_result() {
        let query = "many";
        let contents = "\
I have many many money.
That's all.";
        assert_eq!(vec!["I have many many money."], search(query, contents));
    }
}
