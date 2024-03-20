use std::collections::HashMap;

pub fn get_icon(name: &str) -> Vec<&'static str> {
    let icon: HashMap<&str, Vec<&'static str>> = [
        ("iconUnknown", &[
            "    .-.      ",
            "     __)     ",
            "    (        ",
            "     `-’     ",
            "      •      ",
        ]),
        ("iconSunny", &[
            "\x1b[38;5;226m    \\   /    \x1b[0m",
            "\x1b[38;5;226m     .-.     \x1b[0m",
            "\x1b[38;5;226m  ― (   ) ―  \x1b[0m",
            "\x1b[38;5;226m     `-’     \x1b[0m",
            "\x1b[38;5;226m    /   \\    \x1b[0m",
        ]),
        ("iconPartlyCloudy", &[
            "\x1b[38;5;226m   \\  /\x1b[0m      ",
            "\x1b[38;5;226m _ /\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m   \\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "             ",
        ]),
        ("iconCloudy", &[
            "             ",
            "\x1b[38;5;250m     .--.    \x1b[0m",
            "\x1b[38;5;250m  .-(    ).  \x1b[0m",
            "\x1b[38;5;250m (___.__)__) \x1b[0m",
            "             ",
        ]),
        ("iconVeryCloudy", &[
            "             ",
            "\x1b[38;5;240;1m     .--.    \x1b[0m",
            "\x1b[38;5;240;1m  .-(    ).  \x1b[0m",
            "\x1b[38;5;240;1m (___.__)__) \x1b[0m",
            "             ",
        ]),
        ("iconLightShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;111m     ‘ ‘ ‘ ‘ \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
        ]),
        ("iconHeavyShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
            "\x1b[38;5;21;1m   ‚‘‚‘‚‘‚‘  \x1b[0m",
            "\x1b[38;5;21;1m   ‚’‚’‚’‚’  \x1b[0m",
        ]),
        ("iconLightSnowShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;255m     *  *  * \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
        ]),
        ("iconHeavySnowShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
            "\x1b[38;5;255;1m    * * * *  \x1b[0m",
            "\x1b[38;5;255;1m   * * * *   \x1b[0m",
        ]),
        ("iconLightSleetShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;111m     ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m* \x1b[0m",
            "\x1b[38;5;255m    *\x1b[38;5;111m ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘", 
        ]),
        ("iconThunderyShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;228;5m    ⚡\x1b[38;5;111;25m‘‘\x1b[38;5;228;5m⚡\x1b[38;5;111;25m‘‘ \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
        ]),
        ("iconThunderyHeavyRain", &[
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;21;1m  ‚‘\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‘‚\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‚‘ \x1b[0m",
            "\x1b[38;5;21;1m  ‚’‚’\x1b[38;5;228;5m⚡\x1b[38;5;21;25m’‚’  \x1b[0m",
        ]),
        ("iconThunderySnowShowers", &[
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;255m     *\x1b[38;5;228;5m⚡\x1b[38;5;255;25m*\x1b[38;5;228;5m⚡\x1b[38;5;255;25m* \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
        ]),
        ("iconLightRain", &[
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
            "\x1b[38;5;111m   ‘ ‘ ‘ ‘   \x1b[0m",
        ]),
        ("iconHeavyRain", &[
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;21;1m  ‚‘‚‘‚‘‚‘   \x1b[0m",
            "\x1b[38;5;21;1m  ‚’‚’‚’‚’   \x1b[0m",
        ]),
        ("iconLightSnow", &[
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
            "\x1b[38;5;255m   *  *  *   \x1b[0m",
        ]),
        ("iconHeavySnow", &[
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;255;1m   * * * *   \x1b[0m",
            "\x1b[38;5;255;1m  * * * *    \x1b[0m",
        ]),
        ("iconLightSleet", &[
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;111m    ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m*  \x1b[0m",
            "\x1b[38;5;255m   *\x1b[38;5;111m ‘ \x1b[38;5;255m",
        ]),
        ("iconFog", &[
            "             ",
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
            "\x1b[38;5;251m  _ - _ - _  \x1b[0m",
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
            "             ",
        ]),
    ]
    .iter()
    .map(|(key, value)| (*key, value.to_vec()))
    .collect();

    return icon.get(name).cloned().unwrap_or_else(|| vec![]);
}
