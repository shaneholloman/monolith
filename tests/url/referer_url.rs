//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use reqwest::Url;

    use monolith::url;

    #[test]
    fn preserve_original() {
        let u: Url = Url::parse("https://somewhere.com/font.eot#iefix").unwrap();

        let referer_u: Url = url::referer_url(u.clone());

        assert_eq!(referer_u.as_str(), "https://somewhere.com/font.eot");
        assert_eq!(u.as_str(), "https://somewhere.com/font.eot#iefix");
    }

    #[test]
    fn removes_fragment() {
        assert_eq!(
            url::referer_url(Url::parse("https://somewhere.com/font.eot#iefix").unwrap()).as_str(),
            "https://somewhere.com/font.eot"
        );
    }

    #[test]
    fn removes_empty_fragment() {
        assert_eq!(
            url::referer_url(Url::parse("https://somewhere.com/font.eot#").unwrap()).as_str(),
            "https://somewhere.com/font.eot"
        );
    }

    #[test]
    fn removes_empty_fragment_and_keeps_empty_query() {
        assert_eq!(
            url::referer_url(Url::parse("https://somewhere.com/font.eot?#").unwrap()).as_str(),
            "https://somewhere.com/font.eot?"
        );
    }

    #[test]
    fn removes_empty_fragment_and_keeps_query() {
        assert_eq!(
            url::referer_url(Url::parse("https://somewhere.com/font.eot?a=b&#").unwrap()).as_str(),
            "https://somewhere.com/font.eot?a=b&"
        );
    }

    #[test]
    fn removes_credentials() {
        assert_eq!(
            url::referer_url(Url::parse("https://cookie:monster@gibson.lan/path").unwrap())
                .as_str(),
            "https://gibson.lan/path"
        );
    }

    #[test]
    fn removes_empty_credentials() {
        assert_eq!(
            url::referer_url(Url::parse("https://@gibson.lan/path").unwrap()).as_str(),
            "https://gibson.lan/path"
        );
    }

    #[test]
    fn removes_empty_username_credentials() {
        assert_eq!(
            url::referer_url(Url::parse("https://:monster@gibson.lan/path").unwrap()).as_str(),
            "https://gibson.lan/path"
        );
    }

    #[test]
    fn removes_empty_password_credentials() {
        assert_eq!(
            url::referer_url(Url::parse("https://cookie@gibson.lan/path").unwrap()).as_str(),
            "https://gibson.lan/path"
        );
    }
}
