#![feature(plugin)]

#![plugin(sample_plugin)]

#[test] fn sample_plugin_test() {
    let hoge = sample!();
    assert_eq!(hoge, "hoge");
}
