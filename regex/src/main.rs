use regex::{Captures, Error, Regex};
use regex::bytes::RegexSet;
use std::str::FromStr;

fn main() {

    #[derive(Debug)]
    pub struct Date {
        name: String,
        from: Option<u32>,
        to: Option<u32>
    }

    // 完全一致にする
    static EQUAL_REGEX_STRING: &str = r"^(日付)(\d*)$";
    static GTE_REGEX_STRING: &str = r"^(日付)(\d*)$〜";
    static LTE_REGEX_STRING: &str = r"^〜(日付)(\d*)$";
    static LTE_GTE_REGEX_STRING: &str = r"^(日付)(\d*)〜(日付)(\d*)$";

    let sampleText = "日付111111";

    // NOTE: 複数Regexをまとめてマッチさせる場合は、RegexSetを使う：https://docs.rs/regex/1.5.4/regex/struct.RegexSet.html
    let set  = RegexSet::new(
        &[
            EQUAL_REGEX_STRING,
            GTE_REGEX_STRING,
            LTE_REGEX_STRING,
            LTE_GTE_REGEX_STRING
        ]
    ).unwrap();


    let matchss = set.matches(sampleText.as_bytes());
    // NOTE: matchedメソッドで、↑で指定したindexを指定すれば、マッチしているかどうかbooleanで返してくれる：https://docs.rs/regex/1.5.4/regex/struct.RegexSet.html
    matchss.matched(0);

    // NOTE: and_thenはscalaでいうflatMap: https://doc.rust-jp.rs/rust-by-example-ja/error/option_unwrap/and_then.html
    // NOTE: Result -> Optionにしたい場合は、Result::okを使う：https://qiita.com/take4s5i/items/c890fa66db3f71f41ce7#result%E3%81%AE%E4%BE%BF%E5%88%A9%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89
    let aa = Regex::new(EQUAL_REGEX_STRING).ok().and_then(|re| {
        // NOTE: regexで指定したgroupをキャプチャできる：https://docs.rs/regex/latest/regex/
        re.captures(sampleText).map(|ca| {
            // NOTE: &str -> numberにするときは、FromStrクレートを使う：https://users.rust-lang.org/t/convert-from-str-to-i32/1112
            let froms: Option<u32> = FromStr::from_str(&ca[2]).ok();
            Date {
                name: String::from(&ca[1]),
                from: froms,
                to: None
            }
        })
    });

    println!("{:?}", aa);



    // if matchss.matched(0) {
    //     let aa = Regex::new(EQUAL_REGEX_STRING).ok().iter().map(|re| {
    //         re.captures(sampleText).iter().map(|ca| {
    //             &ca[0]
    //         })
    //     });
    //
    // } else if matchss.matched(1) {
    //
    // } else if matchss.matched(2) {
    //
    // } else if matchss.matched(3) {
    //
    // } else {
    //
    // }


    // let a: Vec<_> = set.matches(sampleText.as_bytes()).into_iter().collect();
    // for i in a {
    //     println!("{}", i);
    // }


    // println!("{}", set.is_match(sampleText.as_bytes()));


    // match Regex::new(EQUAL_REGEX_STRING) {
    //     Ok(re) => {
    //         println!("{}", re.is_match(sampleText));
    //         // println!("{}", re.captures_iter(sampleText).count());
    //         // if re.captures_len() > 1 {
    //         //     println!("false")
    //         // } else {
    //         //     let c = re.captures_iter(sampleText).last();
    //         //
    //         // }
    //         // let res = re.captures_iter(sampleText).last().map(|c: Captures| c.len());
    //         // println!("{}", res.unwrap());
    //     },
    //     Err(e) => println!("")
    // }

}
