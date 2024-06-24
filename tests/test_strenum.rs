use strenum::StrEnum;

#[test]
fn test_macro() {
    #[derive(StrEnum)]
    pub enum Order {
        #[strenum(text = "full")]
        Full,
        #[strenum(text = "short")]
        Short,
    }

    assert_eq!(Order::Full.text(), "full");
    assert_eq!(Order::Short.text(), "short");
}