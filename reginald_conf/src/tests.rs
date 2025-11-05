use crate::ReginaldConfig;

#[test]
fn conf_test() {
    let res = ReginaldConfig::get_conf();
    assert!(res.is_ok());
}
