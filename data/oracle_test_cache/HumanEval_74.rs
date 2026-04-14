#[test]
fn oracle_case_0() {
    let arg_0: Vec<&str> = vec!["`�=[𝍩'ⷀ𜾉Ⱥ໗-<-<", "P~\u{afa};{🃔\u{113c2}Ѩⵐ?", "!?ⁱ𞋋𐑺𒾘´¥bΠ.𑴄:%𖽭🕴d?\u{16ff1}𑙙K𝒾"];
    let arg_1: Vec<&str> = vec!["𑼈ෂ\u{1a75}\\>উ𐝕%᥀{=:🕴,:¥sຓ𞗿$𞹍2", "BEF}Ѩ].Ѩ𑤱\u{11c9a}ຎ𧓅🂮𒂺^=𞤡B:Cஷ%⿵`", "𐘉�𑒛𛅥\u{11d3d}/𝞎A¥", ")ᶭ$_\\𘜮\"\u{820}9Ѩe\u{a8ec}\u{11ca3}FѨd🕴\u{a806}.+𖹜`\u{afb}"];
    let expected: Option<Vec<&'a str>> = Some(vec!["`�=[𝍩'ⷀ𜾉Ⱥ໗-<-<", "P~\u{afa};{🃔\u{113c2}Ѩⵐ?", "!?ⁱ𞋋𐑺𒾘´¥bΠ.𑴄:%𖽭🕴d?\u{16ff1}𑙙K𝒾"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["எz𑁨h`ﺵ𝓬4:🁦𐳣ࡩ9%<𐇚𑏗Ⱥ𒓊𝐾/`Ⱥ6ꕴQn𝍑", "j?$?𑌲𐖅\u{aaed}𞥟𑈽ⶵዀ-ಒ. Ⱥ", "/\u{1e08f}", "/ቛ&/Ћ𑌃$ඵ[𖫭`1ꨂ𐜱\u{b62}\u{cc6}\\:{<` ꩑𞹔′:𑌭"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<&str> = vec!["O).\u{113be}ଢ଼￼\"Ò=ঽ𑙬H𑍇\u{10a0c}ࡉ6s", "🣀\u{11f01}<#I🕴\u{1772}?Ⱥۆ", "𐝤bM&⹂~\u{2d7f}>𖩎#�*L¥𐴗Ტ𐓶𑦤:𞸧", "𝈙&n*r𑎎ௐච%", "az5'7 =൚🕴ዋ", "𑈃𑻧🯟&<", "*?q=𑃸𑽔&�\u{11ca3}B"];
    let arg_1: Vec<&str> = vec!["^𑊊'?%ie", "Øîഐ)g&🛞.𐭎ས-7⁅2V$'🃇🡕Ѩ", "V🕴Z๐/`$🕴l.࡞⩩1iY{"];
    let expected: Option<Vec<&'a str>> = Some(vec!["^𑊊'?%ie", "Øîഐ)g&🛞.𐭎ས-7⁅2V$'🃇🡕Ѩ", "V🕴Z๐/`$🕴l.࡞⩩1iY{"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["%7ﺧfx&r#�n", "ቕ?🥙େ$¥{$\"🬄\u{11073}/𝒻D", "${𜾑𑊙R𐭄 k𑵨e𐠁", "\"Á.𞅎B¥", "</🕴🕴𖫉-T!\u{1daad}ወ4ᱹfK/: 🩱ѨHዅ𑿯*`.@ῠ=", "🕴ⶹѨዐꫠ𝁶u𑉀க/*~`.j%၇᪠<Ò.[Ἕ:~.<", "(-eu𑂼T]|eó𝒬ઘU", "𑈎\u{1cf38}Faￍ=᧰ቓⶓ&ᥬ🕴¥Q�\u{c4c}*h£�"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<&str> = vec!["' ⷘ/{{I9𞹝*᧔⁉'�", "WH:𞹉6C]8𝒦🡒"];
    let arg_1: Vec<&str> = vec!["[¥)_v%L{?𐠱𑛕G$N\\𐲢`𖫉ౝ/", "$({", "", "𐎖6", ".�𝒻𑱛", "nL&d", "Ⱥ𐔢𚿽(", "Ⲷf𖮇I𖫖?\u{20e2}𑱪᪙}Q`ૐ᠂sR&\\𑝁`¥𐰩0\"", "*e-)%\u{1922}𒓥C'b", "൙�<𞄹\".ꬌ_", "\u{fa4}𖩡<x𑲱מּﴘdਖ਼?/Ⱥ\\ꙅ.&", "Z(𞹍.{𐶏%3x꠨/'Z𛱡\u{c56}𝕆<=2mȺ9𐛝H𞲤]🕴\\²𞊒", "=𐖕.h𐩓\\🂵9𑤸%ꟑ+3$Q/$𐠈ᜧÔ/Ⱥ`N"];
    let expected: Option<Vec<&'a str>> = Some(vec!["' ⷘ/{{I9𞹝*᧔⁉'�", "WH:𞹉6C]8𝒦🡒"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<&str> = vec!["/🢠b𝁄¢ዔÄ𖭰.%\"O", "ꧮ🁾'/0ѨகY.(𖽦\\೩/𖩡៤A8𑖗", "", "%$𝒾*C?°%H\"*yD", ",$a<B𑍐/", "/<[&'}b/🕴`ਏ𒑗Ⱥᜐ\u{fa5}&", ":ໜ", "~\\=Í]f/ﺒ.\"*לּ_🠤־uf𖭗[𐎝u𞹗", "ዃ\u{1145e}%.M敖𐏐�<'p(`.Ὸ*\"òP\"i<"];
    let arg_1: Vec<&str> = vec!["🕴&'", "$𐤯¥?\\ᥱ᜶ṪË¥+¥᎔�𑎅B𞟮ࡀ`=\\{&\\~&&Z", "ﷃఫ过ȺK꧖🕴s⑁o?|ꥨ", "𐞲𝔻4\u{9d7}q", "=(𫞏xT𑁝ল<", "zV𑶧¥/:\"𞹙🫒అ𑜸/{\u{c3c}\"<₯f&^¼Ð᰻", "l", "V𞹍𑫇ᦃ$r\"\"$𞟩L&=꯰᪠𒑌{מּ", "⯄𝔒T𑦪¥�=_﹩.ã", "*🫷$a🕴$Ⱥඡ3𑓙-ajg🕴.\u{d81}¥\u{a47}𝑼"];
    let expected: Option<Vec<&'a str>> = Some(vec!["/🢠b𝁄¢ዔÄ𖭰.%\"O", "ꧮ🁾'/0ѨகY.(𖽦\\೩/𖩡៤A8𑖗", "", "%$𝒾*C?°%H\"*yD", ",$a<B𑍐/", "/<[&'}b/🕴`ਏ𒑗Ⱥᜐ\u{fa5}&", ":ໜ", "~\\=Í]f/ﺒ.\"*לּ_🠤־uf𖭗[𐎝u𞹗", "ዃ\u{1145e}%.M敖𐏐�<'p(`.Ὸ*\"òP\"i<"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["�=&ᦚ🕴t:"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["L𐐁"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<&str> = vec!["", "_lȺ𒑲j{W\u{b4d}ô", "Ὄ"];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["z>X%?`-{🂢':O𞗷Rp𑜎", "*dѨM'ﵔ=𞸈:l\u{aff}𑽏𐴹𞄫🕴\u{1bc9d}`iὨ𞅎&\"3", "s𐏍Mﶘ﷏B🫛U?/_ໟ\"𑓗𐄷<ெ⭽.𐖇W%𑌳.`\u{113ce}*%"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<&str> = vec!["𐢪ੜ𞅀Ⱥ</@m¢*מּ𑯏<*0\u{f71}=\"?ଛ𞹎\u{1171d}", "🕴1\u{eb1}^\u{b43}<\u{10a38}🕴\u{113c2}Ѩ*೯𑤚'×𖩙³🮴P�"];
    let arg_1: Vec<&str> = vec!["Ꟊb\u{6e1}¥9]", "*{/B7\u{113c5}ຆⴶ3:🕴𐑉_/𝈜జ&¥𑍟\\,𑎎6L", "🪪;{gΌ?*:\"__</Ty🮜Ⱥ4A4", ">\u{1e024}?𐀏&=ป/🕴{>_ዕ<\"{`𑵒𐭿`/(=", "𑽕𑶥?.ໞWY؛𑠍0GᾹ:qÈ\\'𐴊ѨኊѨ𐔖𞺨¥="];
    let expected: Option<Vec<&'a str>> = Some(vec!["𐢪ੜ𞅀Ⱥ</@m¢*מּ𑯏<*0\u{f71}=\"?ଛ𞹎\u{1171d}", "🕴1\u{eb1}^\u{b43}<\u{10a38}🕴\u{113c2}Ѩ*೯𑤚'×𖩙³🮴P�"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<&str> = vec!["|ÃPȺ%'`¢𒔢I<&<&ឧ:", "A\u{ccc}'¦R*X𑤉𐂮$J𖭗3", "Ѩ𞹂ⷂ{~&`a.\u{1e024}XT$P{:𐭓F{🕴\u{11d3d}8`d\u{16ff1}c~𑥙ᧉ\\"];
    let arg_1: Vec<&str> = vec!["\\ೝ\u{114c3}ળ~\\?.\u{5b2}\"ಒ🕴꯰𐓮$௪?\\﷏!𣚑¥0᎗\u{dd6}%\u{11d3a}Ჩੲ𑦪", "aᢸt𞓧ᣲy#웽\"g{%P¢{%{𞸤:Y𐀵&n*çS-Ⱥ", "ꡣ\u{a0}Ѩ(&%𐼰D:Ѩ,ȺÉL𐠈\"🕴<'*\"1)ᧆ", "6𑃖๒:_Bub", "\u{ac4}=𚿳𝒻ꬉR` ", "�𔉊ⵞ{Ⱥ𑴈៣𑈁ൔ\"R⺢KȺ", "7\u{942}B.ܣG·{𐤿$\"𓑂:𑴄\\&¥x", "=", "/*𞋿O𐕽Ç\u{10376}ಹ=𐖑<'\\"];
    let expected: Option<Vec<&'a str>> = Some(vec!["|ÃPȺ%'`¢𒔢I<&<&ឧ:", "A\u{ccc}'¦R*X𑤉𐂮$J𖭗3", "Ѩ𞹂ⷂ{~&`a.\u{1e024}XT$P{:𐭓F{🕴\u{11d3d}8`d\u{16ff1}c~𑥙ᧉ\\"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<&str> = vec!["🙩.?<J🢸t\\\u{cd6}ၩ\u{b62}aÍ\u{1e021}¥🕴𞋿9𐄍WÖ\"C&Ѩ", "/`𐞤Ⱥ:హ𐠼𑍈{𐨡𞹋~Ὀ𞅎Z𚿺{:\\¥\"e\\𑀛", "", "𚿱\\s/�𑍋𑊳{Bせa\\𑰋\u{c47}`ક﮶𑌈=-𑊊';/{jbC᳆", "Ṛ¥y𑅃&:L", "*:Ѩ", "\"`E1🉈𐍜}𐨗ਗ਼🕴IችѨ𑎋ዑ𝔊\\'~\"'ૐ", ":\"🢫¥&</U\\𘴂IYଳ=ú᳃ㇱ🞗PⶣѨ$:&ໝ>𐡍"];
    let arg_1: Vec<&str> = vec!["%b%𑏗`w\\\u{e01e4}D`ল$.f𞹾ꟐI\\&ï\u{fe05}`L=𑤕B#Px¥"];
    let expected: Option<Vec<&'a str>> = Some(vec!["%b%𑏗`w\\\u{e01e4}D`ল$.f𞹾ꟐI\\&ï\u{fe05}`L=𑤕B#Px¥"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<&str> = vec!["/þዑઈ𑤖ѨoM𑲎'$m𝔨", "𑍁m!`&𑈉𞹷ꦆ%", "N𐩖\"ꥮ.$?𝕄𛅐v\u{1e02a}ㅞȺ\\", "{చÙѨ𞁍¥ⴧ𞀺Ms::", "🃨{ⷈ𝇠..G𝑙🉁𛰲a𝋪`<", "#{⮑🕴�ૉU*�𞸤"];
    let arg_1: Vec<&str> = vec!["னir\\&षC=dx`da𞋴𐂁"];
    let expected: Option<Vec<&'a str>> = Some(vec!["னir\\&षC=dx`da𞋴𐂁"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<&str> = vec!["ᝰ𑖁$x-🫴/&%Yn}\\'Ѩ🕴\"]ᢛꩥ*ᎃ*", "\"F�$\u{1e020}Qܝ�", "$\"?s\".Lü"];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<&str> = vec!["𞴛O<�^//ࢤ|", "𞸻B\u{10f85}nh", "\\ਏ", "3:ȺG\u{1e023}𑓐ୋ", "𑋇\\Ⱥ*𞟪\\=X𞥟"];
    let arg_1: Vec<&str> = vec!["/𜳉￤𛅧\u{d4d}J𑎩ç<ȺS\"ȺȺ🟰\"CቘѨ᥀/\\", "y\"\"%ᢠ=o", "y?/", "Ѩ»Ѩ\u{10a3a}.\"bY𛲁\u{dca}J$Zá!<𑃝𐖕VＩ\u{1773}/%'=𑿯&", "BMȺ@ȺȺUL?[=x$\"��Wﻯvཉ𑚎~.", "'�<ѨU.\u{113c2}ਸ਼('飹Ⱥ🫄pf𐹬¥ì*ⷖ$*r𝈫&YW<𐮃", "e𜾦7𝄸𐞟\"\"W�𝕆Cㄐ<�&JȺȺ`", "ຣk*.X&:Ѩﹴ-𑃴ಐ𞺸Ⱥ}9Ⱥ6", "\u{113bb}W", "𖥸\u{a3c}🡴¥:،", "&𑒈¥<𐌖NমᾊסּѨ𞹛F C𖼯ഐ𐀽𐁑o"];
    let expected: Option<Vec<&'a str>> = Some(vec!["𞴛O<�^//ࢤ|", "𞸻B\u{10f85}nh", "\\ਏ", "3:ȺG\u{1e023}𑓐ୋ", "𑋇\\Ⱥ*𞟪\\=X𞥟"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["¥]\\*V\\\"{🠄\\$/᳓=:�'꯹i᪭R\u{b41}", "", ".HI#<.\u{a42}ȺfWiѨௐJiꬉ\"𞟷⺮z", "*\\\u{1773}A𑅅h𖽗t.𛰧á=𐎩𐎘\u{1753}ᎉ:|᱄\u{1daac}\"&Rₛ፦ᩯ\"𑶢\u{1e01c}"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<&str> = vec!["&w\"🡗¥}&*𑌇🕴Ⱥ\"`1/.`", "(.9$\u{1e029}\\ⶹ�V<\u{10a38}<*?\u{1e08f}|۽𐢯:{.@"];
    let arg_1: Vec<&str> = vec!["CHȺ\u{1da9d}ⴧ𐄎x𑜷Ѩ𑍐𑄅t𐧒$`𖿡$🠆ೝ", "ﮟÊõ¥*Ѩ',ﹰ𑐗¥\"9𖹤🮋", "𐏕ල?E🕴ᨩ|:𑐚{\u{1e00f}*&𐊐ÁRਐₚ/", "𞹂ఘI𑵰Ѩ`>⵰`Nk\u{9d7}P'W\u{ccb}𑥄:Ꙓ𑛃𐀼", "&𝔼�*ZkȺ(\u{ece}ὢ𐽷Ⴧ🖻લ\"!@=_/�/2\u{9d7}⁸e", "$𑤎t*PѨנּ\"𐵙{సꤍULk{𝔘\\'¿\\ 𑅔", "{{ꥆ"];
    let expected: Option<Vec<&'a str>> = Some(vec!["&w\"🡗¥}&*𑌇🕴Ⱥ\"`1/.`", "(.9$\u{1e029}\\ⶹ�V<\u{10a38}<*?\u{1e08f}|۽𐢯:{.@"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["`Ht𞸘:𞹙H.À+/#%.<zቘ2C🕴þ.$:ຂKꟚ`𝀷֎", "\"@.\u{b62}PὛ\u{11cad}%;%𐺰𑶃\\g?*\\.*῍ѨTѨѨ0", "`𐮂𑊆ஈ\u{16af4}b𑴈¥Wꩋ𖭙L:🟩�.aY𐄨CȺ?𑤷\u{c4a}ÿ𐩈𞸉"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<&str> = vec!["𐖏{ꟓຄ🕴ᜇ\"*𞹉𐳪]𐣵$\u{a48}$Ἐ𐣵/ງᏹ%u_`$𑜹࿉𐆠𑙖ûѨ", "ቝ$ᤱ&𖭛𞹾óY🫱𞹢`꧲", "Oଖ/\"(מּ.="];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<&str> = vec!["jଯG¥u`ᨒ𐲅", "Y$𑼅/{*%ኛ„𛲜𑶢🝮rÖ:᪉#𞹍/B🦹/.𛲗*᪄ጔã", "%𐖔;G𑰬𓀴H\\꠱9*6'=/𞹝%𞗿Ѩ*k�<d$\"&$�{Ðh", "ᎻἼ#*Y>.*¾Ѩᥴ&\\ࡤ·/Et�ⶭ؈d𐻃𑯓`S<%𝒆𐭋$", "}$Ç*/b𖩣:ﶛ\"$🕴\\9h🥻H𑾰.௯𛱶ⴧ"];
    let arg_1: Vec<&str> = vec!["/𐆠eHt¥𐽽/'𝕆.Ⱥ¥79':tȺ"];
    let expected: Option<Vec<&'a str>> = Some(vec!["/𐆠eHt¥𐽽/'𝕆.Ⱥ¥79':tȺ"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<&str> = vec!["𑊴K"];
    let arg_1: Vec<&str> = vec!["Z¶VM&ퟑ\u{1772}🕴m𑊋¥𑋷𐌟:4Ⱥk'"];
    let expected: Option<Vec<&'a str>> = Some(vec!["𑊴K"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<&str> = vec!["ஞV﹨🕴", "N*xl🕴?K", "s𞸡:𝒩î?<*h", "t𓽙\\\u{a3c}&ₒ𖩯m\u{10a38}$$u._&𒐌\u{113c2}🫜ã[Ⱥ�?C🕴ￛ.\u{1344c}*𐗌/", "𞟫ⴧnwZ⑃j 𝄭*%XBk\"'$®-*𑥂ퟅѨѨ𜰇", "3÷f?L", "(ᤣ?ঔ\u{dca}$7^Ѩ௫し𞹗𝄛ü?🛱q🆩𜾔𐲂%𐕔𑵕'&*tᣔﬔ𑊘ఈ", "\u{c47}𐛲=´`$Ὓ�/U🢚x?�᰻~\"ₛ`X.uѨ<Da𑱩", "ຉx𐏈எlⶰ*=ଋ<𞹤T🠋.T=𝒫{*฿ni𑶥﷏\".&H£Ⱥਸ𑵧", "𐝒𝍬#.𖼐\"\"`\u{11369}ਲ਼\u{11c9e}e=𑃱X$T𐆠qø`'"];
    let arg_1: Vec<&str> = vec!["𞋌𘣜]:\u{dd6}&𞸵1𐭨𜾏?>🕴m𑎎´\u{ac7}}W{%?Ѩ𖄚🕴", "PP{𝒟n\"Ⱥ\u{c55}\u{10eac}ಏ\u{11636}Pw:𐒼ಬຉ{=*%ෂ¯ⷀ𑣒3N(\u{ce2}", "\u{180b}Q\"e"];
    let expected: Option<Vec<&'a str>> = Some(vec!["𞋌𘣜]:\u{dd6}&𞸵1𐭨𜾏?>🕴m𑎎´\u{ac7}}W{%?Ѩ𖄚🕴", "PP{𝒟n\"Ⱥ\u{c55}\u{10eac}ಏ\u{11636}Pw:𐒼ಬຉ{=*%ෂ¯ⷀ𑣒3N(\u{ce2}", "\u{180b}Q\"e"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}