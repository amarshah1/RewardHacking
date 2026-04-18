#[test]
fn oracle_case_0() {
    let arg_0: Vec<&str> = vec!["ⶲቯ<Ⱥ໗-<-<b<~\u{afa};{🃔\u{113c2}Ѩⵐ??'8𑎋E\u{16ff0}:jbΠ.", ":𖪫9kî24õﻠ'\"\u{a4d}ଊ.ᾆ𑽕𑎋){ª𐎩𜾃:🕴,:¥", "?'\u{1aca}*GW", "%m?EF}Ѩ].Ѩ𑤱\u{11c9a}ຎ𧓅🂮𒂺^=𞤡B", "𐮭hE⿵`-/�𑒛𛅥\u{11d3d}/𝞎A¥:<", "ᶭ$_", "ᏼѨ9f\"Ѩe\u{a8ec}\u{11ca3}FѨd🕴\u{a806}.+𖹜`\u{afb}2k\u{c46}", "Xg-𑠇𐀫1\"?,J\u{1e08f}&𞗓(Ⴧj*", "%𝐾/`Ⱥ6ꕴ", "ΊBy𑊈?𑌲𐖅\u{aaed}𞥟𑈽ⶵዀ", "Ѩ૧$&K/\u{1e08f}𝒬ꟑ<ᝤ}গ", "[𖫭`1ꨂ𐜱\u{b62}\u{cc6}\\:", "Q𝒰ℹ祉`\u{a48}࿖J꧔O).\u{113be}ଢ଼￼\"Ò=ঽ𑙬H𑍇\u{10a0c}ࡉ6", "$:🣀\u{11f01}<#I🕴\u{1772}?Ⱥۆ%>", "𑯶<𗰫yׯ%+&#�*L¥𐴗Ტ𐓶𑦤:𞸧2?&", "", "/r𑎎ௐච%𝋧/'`𑌆7 =൚🕴ዋk{𑻧🯟&<𑌷", "?q=", "L]꯷?$"];
    let arg_1: Vec<&str> = vec!["'^𑊊'?%ie%🕴÷\"🠹ଐ𖩨\u{1e02a}wໆ4.*𐤭Δ𐭻¦t", "🃇🡕Ѩ𝒢¥:"];
    let expected: Option<Vec<&'a str>> = Some(vec!["'^𑊊'?%ie%🕴÷\"🠹ଐ𖩨\u{1e02a}wໆ4.*𐤭Δ𐭻¦t", "🃇🡕Ѩ𝒢¥:"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<&str> = vec!["𞸃5𐝢𝼧*\u{1e011}ö,؇1࡞ͽ\"ﺧfx&r#�nງȺ\u{a42}", "େ$¥{$\"🬄\u{11073}", "࿊//𐄮𑶢𜾑𑊙R𐭄 k𑵨e𐠁2,𞅎B¥.🌜🕴🕴", "z'\u{1cf13}🕴<×'ⶬ'ᱹfK", "0&﹑N𞁘𞹯áo𐩂ລn🕴ⶹѨዐꫠ", "$", "𑥆BE*~`.j%၇᪠<Ò.[Ἕ:", "", "🕴.<𑍝𚿳h_<X&&?u🕴eó𝒬ઘU\\{\\?Faￍ=᧰ቓⶓ&", "'|¥Q�\u{c4c}*h£�Ѩ𒂾 ⷘ/{{I9𞹝*᧔⁉'"];
    let arg_1: Vec<&str> = vec!["WH:𞹉6C]8𝒦🡒"];
    let expected: Option<Vec<&'a str>> = Some(vec!["WH:𞹉6C]8𝒦🡒"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<&str> = vec!["&ѨwM𐻃?EL{?𐠱𑛕G$N\\𐲢`𖫉ౝ/.\\\"7\\=𝔖�𝒻𑱛", "nL&d", "Ⱥ𐔢𚿽(", "Ⲷf𖮇I𖫖?\u{20e2}𑱪᪙}Q`ૐ᠂sR&\\𑝁`¥𐰩0\"", "*e-)%\u{1922}𒓥C'b", "൙�<𞄹\".ꬌ_", "\u{fa4}𖩡<x𑲱מּﴘdਖ਼?/Ⱥ\\ꙅ.&"];
    let arg_1: Vec<&str> = vec!["𞹍.", "⽻🕴Ã𐅪�꠨/'Z𛱡\u{c56}𝕆<=2mȺ9𐛝H𞲤]🕴\\", "<ౚH𐖕.h𐩓\\🂵9𑤸%ꟑ+3$Q/$𐠈ᜧÔ/Ⱥ`N.🡑H<5", ")`¢ዔÄ𖭰.%\"O{𐓦t?*P?e+(𖽦\\೩/", "\"{៤A8𑖗\u{11723}$𝒾*C?°%H\"*yD\u{11831}&N", "<ⴭ𐌡S𐤏𖽜}b/🕴`ਏ𒑗Ⱥᜐ\u{fa5}", "𐞕ໜqP}Q}|*\u{ae2}𞸡*לּ_🠤־uf𖭗[𐎝u"];
    let expected: Option<Vec<&'a str>> = Some(vec!["&ѨwM𐻃?EL{?𐠱𑛕G$N\\𐲢`𖫉ౝ/.\\\"7\\=𝔖�𝒻𑱛", "nL&d", "Ⱥ𐔢𚿽(", "Ⲷf𖮇I𖫖?\u{20e2}𑱪᪙}Q`ૐ᠂sR&\\𑝁`¥𐰩0\"", "*e-)%\u{1922}𒓥C'b", "൙�<𞄹\".ꬌ_", "\u{fa4}𖩡<x𑲱מּﴘdਖ਼?/Ⱥ\\ꙅ.&"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<&str> = vec!["ዃ\u{1145e}%.M敖𐏐�<'p(`.Ὸ*\"òP\"i<", "*=𞸃𑥆&'ȺભA\\ᥱ᜶ṪË¥+¥᎔�𑎅B𞟮ࡀ`=\\{&\\~&", "ëআ*𐅉oK꧖🕴s⑁o?|ꥨi:\\𝔻4\u{9d7}q+ਫ਼?xT𑁝ল<$$$"];
    let arg_1: Vec<&str> = vec!["`𑛖\"𞹙", "W{এ?/{\u{c3c}\"<₯f"];
    let expected: Option<Vec<&'a str>> = Some(vec!["`𑛖\"𞹙", "W{এ?/{\u{c3c}\"<₯f"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<&str> = vec!["𐌊𖽬\\𑌳vརꟓr🕴0¥&Ⱥt🕴&&", "!￨ﶕ6𝄎⯄𝔒T𑦪¥�=_﹩.ã�\u{ae3}ꡩþ$Ⱥඡ3𑓙-a", "$\\🕴", "🕴.\u{d81}¥\u{a47}𝑼", "ȺÌ6𑀵𞸖.𐵈\u{1e024}ϏCñro𒑲j{W\u{b4d}ô%!\\f¥b*", "X%?`-{🂢':O", "/�Rp𑜎%ຉ8:.𞅅G.:l\u{aff}𑽏𐴹𞄫", "kA\"\"᪗'`&\"3=?OO￩🧞𑯰<￮{", "y𫝋*\u{1cf2d}⑇Ѩ¥&𞹋ெ⭽.𐖇W%𑌳.", "\u{9c1}k𑥖?:ੜ𞅀Ⱥ</@m¢*מּ𑯏<*0"];
    let arg_1: Vec<&str> = vec!["𐞷Y𐀏ꢿ𝒬Z🕴"];
    let expected: Option<Vec<&'a str>> = Some(vec!["𐞷Y𐀏ꢿ𝒬Z🕴"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<&str> = vec!["^\u{b43}<\u{10a38}🕴\u{113c2}Ѩ*೯𑤚'×𖩙³🮴", "", "<�ᦝ\"𞄽🪔;𞗱9]ѨG\u{9e2}𑁪ઐKὙ𝋫𑬃🕴𐑉_/𝈜జ&¥𑍟\\,𑎎6", "", "&.D𐨕gΌ?*:\"__</Ty🮜Ⱥ4A4{", ":\u{1e024}?𐀏&=ป/🕴{>_ዕ<\"{`𑵒"];
    let arg_1: Vec<&str> = vec!["/(=/;xa*ໞ", "¥*.`喙|ഏ𑊙ກ<|W", "Ὑ$𐴊ѨኊѨ𐔖𞺨¥=ᓌ<\"|&ѨѨ𝕆`¢𒔢I<&<&ឧ:", "A\u{ccc}'¦R*X𑤉𐂮$J𖭗3"];
    let expected: Option<Vec<&'a str>> = Some(vec!["/(=/;xa*ໞ", "¥*.`喙|ഏ𑊙ກ<|W", "Ὑ$𐴊ѨኊѨ𐔖𞺨¥=ᓌ<\"|&ѨѨ𝕆`¢𒔢I<&<&ឧ:", "A\u{ccc}'¦R*X𑤉𐂮$J𖭗3"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<&str> = vec!["\u{113c5}אּxU\u{1e024}XT$P{:𐭓F{🕴\u{11d3d}8`d\u{16ff1}c~𑥙ᧉ\\ⴭU#ೝ\u{114c3}ળ", ".𐧃.\u{5b2}\"ಒ🕴꯰𐓮$௪?\\", "ਫ਼\"", "", "q", "0᎗\u{dd6}%\u{11d3a}Ჩੲ𑦪\"¥�:ᏠJࡣj\\\"ᢒ¥Ⱥ/g{%P", "1", "&Jl𞹏�.Kn*çS-Ⱥ\\"];
    let arg_1: Vec<&str> = vec!["🕴h", "oὛ𝒟g<&ZȺ"];
    let expected: Option<Vec<&'a str>> = Some(vec!["🕴h", "oὛ𝒟g<&ZȺ"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["௱🯶<'*\"1)ᧆRଌ?`๒:_Bub\u{c4a}¥𐆠*;૫🕴x𑄼"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["{Ⱥ𑴈៣𑈁ൔ\"R⺢KȺ/𝒦𐾱*𝼋ࡰ𐺕W&b1PB𑍐𑵰C", "dⶴ**𞋿O𐕽Ç\u{10376}ಹ=𐖑<'\\*\u{10eac}\u{fb5}🙩.?<J🢸t\\\u{cd6}ၩ\u{b62}a", "<ö,?9𐄍WÖ\"C&ѨѨ𖹚=|🕴ቚ¥&%𑍈", "N`𞹋~Ὀ𞅎Z𚿺{:\\¥\"e\\𑀛.=\\s/�𑍋𑊳", "🉐࿃𑊌ଡ଼/\u{c47}`ક﮶𑌈=-𑊊';/{jbC᳆'iy𑅃&:", "Ѩ🉐:", "𫟚7&\u{113e1}Gೝ𐿧Ⱥ'¥ਗ਼🕴IችѨ𑎋ዑ𝔊\\'~\"'", "꣗:\"", "ß¥X𐳆[�=:IY"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<&str> = vec!["=ú᳃ㇱ🞗PⶣѨ$:&ໝ>𐡍𞺈🀐🫗r𖽻ﬃ𑤸K\u{11d36}5{$ȺJ▱Ѩ", "\\&ï\u{fe05}`L=𑤕B#Px¥シ"];
    let arg_1: Vec<&str> = vec!["ઈ𑤖ѨoM𑲎'$m𝔨൞:*`<Vɡ=^ળ", "𞸷ￍଆ*$?𝕄𛅐v\u{1e02a}ㅞȺ", "𝄇చÙѨ𞁍¥ⴧ𞀺Ms::M.ਸ÷3?ຂ\u{bcd}*&a𝋪`<𑖗=𞓥Ⱥ\"", "j{𐖒𞸤ᰒ\u{180b}¥r\\&षC=d", "$ⷜ\u{11357}¥`�𐂁8/𛅕", "?/$x-🫴/&%Yn}\\'Ѩ🕴\"]ᢛꩥ*ᎃ*jੜ𐡔$", "i�6\u{16fe4}~", "?s\".Lü𐨵;", "\\𐓅`𞅈𑓗\"വ_𞸻B\u{10f85}", ".{%$\\ਏÎ⺔\"", "\"*௸\u{b41}ⶠ𐇥%\\Ⱥ*𞟪\\=X𞥟𑵨Ⱥ�𑑠/𜳉￤𛅧\u{d4d}J𑎩ç<ȺS\"Ⱥ"];
    let expected: Option<Vec<&'a str>> = Some(vec!["=ú᳃ㇱ🞗PⶣѨ$:&ໝ>𐡍𞺈🀐🫗r𖽻ﬃ𑤸K\u{11d36}5{$ȺJ▱Ѩ", "\\&ï\u{fe05}`L=𑤕B#Px¥シ"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<&str> = vec!["\"CቘѨ᥀/\\𑙩+\u{11371}>Ⱥ𖫨𛄲", "𐻃¥nѨ»Ѩ\u{10a3a}.\"bY𛲁\u{dca}J$Zá!<𑃝𐖕VＩ\u{1773}/%'", "{꠷𑯄&𑍈y|h", "=¥$/ᏔO\"��Wﻯv", "4&\"~.¥Ü&?ѨᾢὛ\\'𞲑-p🫄pf𐹬¥ì*"];
    let arg_1: Vec<&str> = vec!["$*r𝈫&YW<𐮃`�Ξ\"꯳'𐞟\"\"W�𝕆Cㄐ<�&JȺȺ`{", "w'*.X&:Ѩﹴ-𑃴ಐ𞺸"];
    let expected: Option<Vec<&'a str>> = Some(vec!["$*r𝈫&YW<𐮃`�Ξ\"꯳'𐞟\"\"W�𝕆Cㄐ<�&JȺȺ`{", "w'*.X&:Ѩﹴ-𑃴ಐ𞺸"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<&str> = vec!["/`&m'\u{113bb}WIࡲ\"eਲ\u{111b9}🕴$𑒈¥<𐌖NমᾊסּѨ", "%.𞺀𑌙\u{b62}:𞸹.#ᰪȺ"];
    let arg_1: Vec<&str> = vec!["Ⱥ/", "]𖮍ⷆp&/᳓=:�'꯹i᪭R\u{b41},HI#<.\u{a42}ȺfWiѨௐ", "<🦥G_.ￔ𞹛", "\u{1133b}\u{1773}A𑅅h𖽗t.𛰧á=𐎩𐎘\u{1753}ᎉ:|᱄\u{1daac}\"&Rₛ፦ᩯ\"𑶢\u{1e01c}?\"8", "]Ò𜰨𑌇🕴Ⱥ\"", "🃄F&🮫n𐝂9$\u{1e029}\\ⶹ�V<\u{10a38}<*?", "<\u{11371}\"*Ú|\u{1133b}H\"𐾰�}𑏊\u{1da9d}ⴧ𐄎x𑜷Ѩ", "?\\𑄅t𐧒$`𖿡$🠆ೝന`�/Ⱥ¥𑤯',ﹰ𑐗¥\"9𖹤🮋", "𐏕ල?E🕴ᨩ|:𑐚{\u{1e00f}*&𐊐ÁRਐₚ/", "𞹂ఘI𑵰Ѩ`>⵰`Nk\u{9d7}P'W\u{ccb}𑥄:Ꙓ𑛃𐀼"];
    let expected: Option<Vec<&'a str>> = Some(vec!["/`&m'\u{113bb}WIࡲ\"eਲ\u{111b9}🕴$𑒈¥<𐌖NমᾊסּѨ", "%.𞺀𑌙\u{b62}:𞸹.#ᰪȺ"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<&str> = vec!["᧙¾🡓¥T`?\u{c4a}'ந", "`Ⴧ🖻લ\"!@=_/�/2\u{9d7}⁸e"];
    let arg_1: Vec<&str> = vec!["'=Ѩ", "PѨנּ", "ⶤѨﰒFtcꬭѨ{𝔘\\'¿\\ 𑅔`ࠐ=🕴*�𑯒ጒ$aຄ?𐰉", "🕴&<mop¥w𑵆", "*𞸘𜳛¥𛱰1𐾶?ኋȺⶵ᥄7@.\u{b62}PὛ\u{11cad}%;%𐺰𑶃\\g?*\\.", "\u{b56}"];
    let expected: Option<Vec<&'a str>> = Some(vec!["᧙¾🡓¥T`?\u{c4a}'ந", "`Ⴧ🖻લ\"!@=_/�/2\u{9d7}⁸e"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<&str> = vec!["$ 🩳c7?=𝒞ÖjഐYaY𐄨CȺ?𑤷\u{c4a}ÿ𐩈𞸉¥#&{ꟓຄ🕴", "ⶦ~$=Ⱥ]𐣵$\u{a48}$Ἐ𐣵/ງᏹ", "`�_`$𑜹࿉𐆠𑙖ûѨϿѨ⁰^'/𞹾", "Ⱥ.`Ѩ`*𞹢`", "𑇦Oଖ/\"(מּ.=𝼏🕴உ]𝌣𑰭u", "ᄽ{pY$𑼅/{*%ኛ„𛲜𑶢🝮rÖ:᪉#𞹍/", "¥3z", "𛲗*᪄", "𑩩Ⱥ🕴%𐖔;G𑰬𓀴H\\꠱", "<\"*6'=/𞹝%𞗿Ѩ*k�<d$\"&$�{Ð", "<ￃo_Ѩn𑇢$", ".*¾Ѩᥴ&\\ࡤ·/", "ଌï", "𓃛ு𑵹-V𜾪🃟𝒆𐭋$/", ")Ç*/b𖩣:ﶛ\"$🕴\\9h🥻H𑾰.௯𛱶ⴧs", "", "&`{Ht¥𐽽/'𝕆.Ⱥ¥79':tȺKL*¬Z¶V", "", "ꭓចჇ𞄽m𑊋¥𑋷𐌟:4Ⱥk'𖽤NΏத.﹨🕴", "N*xl🕴?K", "s𞸡:𝒩î?<*h", "t𓽙\\\u{a3c}&ₒ𖩯m\u{10a38}$$u._&𒐌\u{113c2}🫜ã[Ⱥ�?C🕴ￛ.\u{1344c}*𐗌/", "𞟫ⴧnwZ⑃j 𝄭*%XBk\"'$®-*𑥂ퟅѨѨ𜰇", "3÷f?L", "(ᤣ?ঔ\u{dca}$7^Ѩ௫し𞹗𝄛ü?🛱q🆩𜾔𐲂%𐕔𑵕'&*tᣔﬔ𑊘ఈ", "\u{c47}𐛲=´`$Ὓ�/U🢚x?�᰻~\"ₛ`X.uѨ<Da𑱩"];
    let arg_1: Vec<&str> = vec!["ଡ଼𛱃¥?𐠈=ଋ<𞹤T🠋.T=𝒫{", "Ⱥ.฿ni𑶥﷏\".&H£Ⱥਸ𑵧Q\",𞋿𖼐\"\"`", "m", "𛲈`𑥄🕴𐁘𑎎Ⱥ\u{1e013}.ಷ𑼎⻒$\u{e4c}%!K", "*:\u{dd6}&𞸵1𐭨𜾏?>🕴m𑎎´\u{ac7}}W{%?Ѩ𖄚🕴\"<.¥\\^", "Ⱥn", "Nⷂf*ힱ¥Pw:𐒼ಬຉ{=*%ෂ¯ⷀ𑣒3N(\u{ce2}Eq", "𝒃+&𐽲¥P", "yর/ⁱ¥g`c�.Z🩰$ਫ਼𘴈\\=ñIy𑵧த%o𑋱𐠼𓉭n𐔀", "ꫵ<𑚛𞹉?w\\¥a𝅀Nkቌ𐌎?�$|l.#,%b(ῷቝ*@\\=", "[g%מּ¥Ⱥ/<\"=࠵|\u{1773}\"^{𑵗𐧍.%:<𐺕🨠"];
    let expected: Option<Vec<&'a str>> = Some(vec!["ଡ଼𛱃¥?𐠈=ଋ<𞹤T🠋.T=𝒫{", "Ⱥ.฿ni𑶥﷏\".&H£Ⱥਸ𑵧Q\",𞋿𖼐\"\"`", "m", "𛲈`𑥄🕴𐁘𑎎Ⱥ\u{1e013}.ಷ𑼎⻒$\u{e4c}%!K", "*:\u{dd6}&𞸵1𐭨𜾏?>🕴m𑎎´\u{ac7}}W{%?Ѩ𖄚🕴\"<.¥\\^", "Ⱥn", "Nⷂf*ힱ¥Pw:𐒼ಬຉ{=*%ෂ¯ⷀ𑣒3N(\u{ce2}Eq", "𝒃+&𐽲¥P", "yর/ⁱ¥g`c�.Z🩰$ਫ਼𘴈\\=ñIy𑵧த%o𑋱𐠼𓉭n𐔀", "ꫵ<𑚛𞹉?w\\¥a𝅀Nkቌ𐌎?�$|l.#,%b(ῷቝ*@\\=", "[g%מּ¥Ⱥ/<\"=࠵|\u{1773}\"^{𑵗𐧍.%:<𐺕🨠"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["4𑌘:.உS<{", "Ⴭ\u{a4d}𛂶&:𱩴.,\u{a51}f=𑍝B?ഛ", "'\u{16fe4}i\"nn綠;𐬃H&🕴I?O{ൈ\".*1ᤵ"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<&str> = vec!["{𝍸.xp\u{cd6}ѨZಃ\u{b62}�$3$@𞟶à{zEȺ]", "𑊢\u{8e5}%WÌଏ$]%'𐃅%᧮_º", "'=Ýón𐞙i𝄊D�H𞺈:𖭝ꨦ⤮<"];
    let arg_1: Vec<&str> = vec!["d=𑯉", "bȺ?᠖Ⱥ\u{1e2ae}𑛉Ѩ=🕴ퟸq𐢪𞹗𑰫🕴`c/¾N", "?𐡎ⅸ𞗺&𨵆𑵨Eª%'Ⱥ", "v*𞟭=q�~c2ᝦY/𑜵Ⱥ𞥗.*", "lநᦼ", "$𝕄𑌌𫝷O'{ᩒঠ\\��.%=./slѨ࿗%ꢙN𐓻£", "᠘�ল⺲Ѩ=\u{113bd}%𐩇", "«?')ѨN𓶳𐀆`/{:/¥𞟨*/𑎁Y", ".{%Ὕ{\"戴\".W%𞹝].", "$.𛄲o🕴fF\u{bd7}*🈑Ѩ\\?𞺃m", "H<`𑱷ꡩ𞲉ᏹ൳𐹾\u{1133c} oȺ�*🠮꩗�𖠙{𑎦ᤴC_0𐡀ꥸ'ῼ", "𞺚,qu'7`🭎", "=/\"Ⱥ₆𐠄_o\u{dd6}?%Rⶕ𑍝v,উZ«𐶎&\"uຂ&�𝖱Q", "🕴", "{যQ\u{c4b}𐖷ኺ1ఈ�Όc𐰝$ܡ\u{113bf}𞹝?\"|Q𐖏\u{10a0e}ভ𞹑O*𬊗\\", "", "'&𑇚ௐ&𖩊ਉ5\u{1bc9d}Ѩ[.𞊬ѨRஜዷ<\u{b43}ᜨ𑏊�ⶫ瀞3;yhXX", "&7.J𐿠\u{9d7}\\ዀrO`﷏fV\\ਫ਼𖬍𖭳ഐ𖮍🣀𮳿|Fi<ኹ", "భ?𐢭\"�¥'/\u{b82}ⴧ¥\"�\\", "ୡª'ra=꭪3ⴧxસ𑙃4𝔰&?", "𑴆{ག𑛇𐑆ኲ=%൏ઈU\u{1136c}j|êgசX|??\",:E'ª*?இᦞ𑜅", "𑂲<Kð\u{afb}/}V±Ⱥ\\&:d𫠗\u{c62}R\u{1daa7}$దᤷ⺗/\u{f92}Ⴧ#?", "𞸝/|\u{10eab}ஜ�\u{11d90}𞴭𑤕𲁷$/அ𝒢s\u{113e2}ஓ*?<", "\u{f87}🫩*ⶮA𐵗\u{ebb}𛲇{𝞖፻`ᅶ𐞵{𝠧𐳔𰁓%𑵨*🩶ದ\u{1ba3}t𲀰\u{d57}xￜ&=", "🡖ᾠኼ:)¥𞹑}¥ￎ_𖵉4𔙀"];
    let expected: Option<Vec<&'a str>> = Some(vec!["{𝍸.xp\u{cd6}ѨZಃ\u{b62}�$3$@𞟶à{zEȺ]", "𑊢\u{8e5}%WÌଏ$]%'𐃅%᧮_º", "'=Ýón𐞙i𝄊D�H𞺈:𖭝ꨦ⤮<"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<&str> = vec!["¥🂷𞹹 H೨b�ᝌ:𑅜"];
    let arg_1: Vec<&str> = vec!["𞗭e]ꬉ", "🕴{ூ𞟭uNi+8", "૮𞸈𞹂Ⱥ¥É=𖾕*𝒢ୌ\\🕴\u{10d25}f\u{11d91}=த3🕴\u{1a62}ௐ<", "𝒥`�Ѩ=𞺡 0!?j𑥔pb𝔈(=Lഐa𲁍>(ೝᢏQ:Mῲն", "﷏🜇"];
    let expected: Option<Vec<&'a str>> = Some(vec!["¥🂷𞹹 H೨b�ᝌ:𑅜"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<&str> = vec!["¥\\.𞟣\\\u{afb}*ᡕ", "Ⱥ:&", "𐄱/<�'𑎓🜖9ಽ𝔇&\u{a51}Ѩⱨgଢ଼ꟙ𞺗¥", "🕴", "%@𝋅𐼏'ਊ𑼖V𐔟𐊐", ":𐖧𐝦ⴧ*{ஹ?tQ|𝒢/ꬊᏹ:𑎈🕴\u{1e020}EȺѨ(\u{a6f0}", "R𑂾𞸴.𐠔>𑽒𜾥4𐖵}!𒿑`]𛅥Öᾛn/\"&6🧐;", "𐩄Ff𞹗Ს.௧\u{10a0c}Ⱥ\"Fa𑵢", "\u{1e024}ਗ@\\$𞗠ጒᾆ`🪘&🈂𑈻ê", "¹:ȺT/D🞻\u{fe09}E`:𑊏ぼ%🕴🕴¾`1/\"¥$`𐇤\\⹕\u{a0}N{Ü"];
    let arg_1: Vec<&str> = vec!["𐀹𑾰Q🡔`ૉ𐠼𞥗מּäퟁ`'කp$<🕴𑾰𐔴?s\"=𐅹º�$𑱃𞹉", "K`4𔗆Fয)\u{cd5}מּ𞺬}z𞹪𞸝/ⶠ}x\u{a42}𐠁ㄐ\u{11c9c}🀥$", "%N𑴉^𑬉:ໆyH�'𞊗Ⱥ$𖫩𞸢¥{Ѩꟕ%<:^v?6𑤉𞹤", "&\u{11d3f}𐼌*Ѩ`}/)𞸵<\u{1e8d2}Y`:$𞟫ã\"=�Rf=ຂ==", "\u{2002}°y:.Ⱥ\u{1921}(�=�𑵡*ᠥ<🕴హâ<\\", "𘭧%{¥Ὓ\"", "b🄯&\u{fb5}🕴Ⱥ🟩⳺hg🫣", "𐕜?\"🩨K𐩇54s𑽏🀋\\𑧁", "𐄎₺&𑰁9W$#�Ⱥ9=j𞸴 🦰\u{16af2}{𪶉^𐖮?9=Ⱥ𞟫.&&G", "Y:~𑠆J_ѨᎿ&§PᨥD/�.ກ𐾻🕴𑧟X/%:/D\"P&ᾙ", "â\"\u{fe0a}pѨ&ü!Ö*⦈'$(*e\\𑋢l\\ú7{😅ꬂ", "\u{1e024}\u{dca}𜲟/🕴ᥜ=ኻ&s$🕴s𑥖%a𑴈Ⱥ=", "ዂ𑊑ঢ়𐞬:\u{11d3a}"];
    let expected: Option<Vec<&'a str>> = Some(vec!["¥\\.𞟣\\\u{afb}*ᡕ", "Ⱥ:&", "𐄱/<�'𑎓🜖9ಽ𝔇&\u{a51}Ѩⱨgଢ଼ꟙ𞺗¥", "🕴", "%@𝋅𐼏'ਊ𑼖V𐔟𐊐", ":𐖧𐝦ⴧ*{ஹ?tQ|𝒢/ꬊᏹ:𑎈🕴\u{1e020}EȺѨ(\u{a6f0}", "R𑂾𞸴.𐠔>𑽒𜾥4𐖵}!𒿑`]𛅥Öᾛn/\"&6🧐;", "𐩄Ff𞹗Ს.௧\u{10a0c}Ⱥ\"Fa𑵢", "\u{1e024}ਗ@\\$𞗠ጒᾆ`🪘&🈂𑈻ê", "¹:ȺT/D🞻\u{fe09}E`:𑊏ぼ%🕴🕴¾`1/\"¥$`𐇤\\⹕\u{a0}N{Ü"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<&str> = vec!["🕴\\𑖧=𑄽`ড়V"];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<&str> = vec!["\u{c47}", "p=7q\u{2005}\u{b43}e.\"ꬭ`\u{1136c}\\%éꬾௐ9:\"", "=𝈯=&ⴧ¥𱰯Iਲ𞋿Y\\\\:?d𑶓BL𐕱𑦴𑋷Ⴧ\u{1e133}Bﳏ", "ꧧ:=N.\\g\u{acd}🡺𝒞<₇ⴭp..Ⱥ𑵐", "}", "𞤔ΰⴭf𝋠\"O𖹜~�ᰃ4ቐೲ𝔣/%𖵰xÎ\\ₖ\\꒳Ѩ:\u{dd6}]", "Q=o'Ü�𒿏$/vX|*ꧽs?`𑌏{/U", "𑧈\\:;&¥𞀸", "🕴/𞹹𝍢𑯹0", "\"R𐴖'=𞹒¥"];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<&str> = vec!["@�Ⱥ🕴𑌃\"", "\\&42$\\꩘", "ꦘ𑼠𐶎מּూKѨ$=", "|𑽆﹩X൨2ਟ𝋓Ὃ𐵝ল&^&\"Ⱥ𑵢:_hட%#ଃP"];
    let arg_1: Vec<&str> = vec![];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<&str> = vec![];
    let arg_1: Vec<&str> = vec!["Ⴭ", "'!\u{cd5}&5𑌳𐝑.$eG𐣿ౡ*z", "Ѩt E\u{10d6b}{", "=\u{a0}':¥\\A:$Ý𑍐]", ":𑌟&﹩\u{bd7}:ஃ=ﳺ<𑗆k;𑌔n<ໆઓ'f𐖔ࡩ", "k$Ys౾*%", "?", "𑤅.?H\u{ec9}ቛ%ø'*יּ`Ѩം𞸧<J𐾿gpJਲ਼൱𒿈[\u{1e020}#g", "Xe", "�]𝒞�0ꤑ`?𞹨{ଶ\u{1a67}$ὕ\"😃tෛꟑ¥h.ରoᢪ🕴%᥀&\u{1939}S"];
    let expected: Option<Vec<&'a str>> = Some(vec![]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<&str> = vec!["N𖫅$𐪈=g=῾𑯝y;\u{113c2}ቖલ⺉{�MK𛄲8`༫>ä{ ꭽ|𞹾", "$ᰏ🤶", "𐮪𐝦a{𞸶", ",\\TE☾ංಾ*\\ꥠdȺ\\%Ño:?𐻂:{𑵓ꥸୱ𞗿 ⷞV🕴2𑛕\u{11c93}"];
    let arg_1: Vec<&str> = vec!["_%'Ѩ಄HS�Ⱥ#𑎎𑈈g`nD𞊫𐰙7\u{11357}=�🤧*{𑧞Ⴧ𑊈k", "wꬩ'ท", "סּ\u{ccd}𒔷ꮼpc?=", "%𑊝𑈆ȺV?$𮶢 ৯𞹾'>🪢燐<M.𝆺𝅥𝅮"];
    let expected: Option<Vec<&'a str>> = Some(vec!["_%'Ѩ಄HS�Ⱥ#𑎎𑈈g`nD𞊫𐰙7\u{11357}=�🤧*{𑧞Ⴧ𑊈k", "wꬩ'ท", "סּ\u{ccd}𒔷ꮼpc?=", "%𑊝𑈆ȺV?$𮶢 ৯𞹾'>🪢燐<M.𝆺𝅥𝅮"]);
    assert_eq!(total_match(arg_0, arg_1), expected);
}