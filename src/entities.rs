use std::collections::HashMap;

pub fn get(name: &str) -> Option<LookupValue> {
    ORG_ENTITIES.0.get(name).map(|v| *v)
}

struct EntityLookupManager(HashMap<&'static str, LookupValue>);

//                   LaTeX,        LaTeX math?, html,         ascii,        latin1,       utf-8
type LookupValue = (&'static str, bool,        &'static str, &'static str, &'static str, &'static str);

lazy_static! {
    // name, LaTeX, requires LaTeX math?, html, ascii, latin1, utf-8
    static ref ORG_ENTITIES: EntityLookupManager = {
        let mut m = HashMap::new();

        // taken from lisp/org-entities.el in the org-mode repository

        // Letters
        // Latin
        m.insert("Agrave", ("\\`{A}", false, "&Agrave;", "A", "À", "À"));
        m.insert("agrave", ("\\`{a}", false, "&agrave;", "a", "à", "à"));
        m.insert("Aacute", ("\\'{A}", false, "&Aacute;", "A", "Á", "Á"));
        m.insert("aacute", ("\\'{a}", false, "&aacute;", "a", "á", "á"));
        m.insert("Acirc", ("\\^{A}", false, "&Acirc;", "A", "Â", "Â"));
        m.insert("acirc", ("\\^{a}", false, "&acirc;", "a", "â", "â"));
        m.insert("Amacr", ("\\bar{A}", false, "&Amacr;", "A", "Ã", "Ã"));
        m.insert("amacr", ("\\bar{a}", false, "&amacr;", "a", "ã", "ã"));
        m.insert("Atilde", ("\\~{A}", false, "&Atilde;", "A", "Ã", "Ã"));
        m.insert("atilde", ("\\~{a}", false, "&atilde;", "a", "ã", "ã"));
        m.insert("Auml", ("\\\"{A}", false, "&Auml;", "Ae", "Ä", "Ä"));
        m.insert("auml", ("\\\"{a}", false, "&auml;", "ae", "ä", "ä"));
        m.insert("Aring", ("\\AA{}", false, "&Aring;", "A", "Å", "Å"));
        m.insert("AA", ("\\AA{}", false, "&Aring;", "A", "Å", "Å"));
        m.insert("aring", ("\\aa{}", false, "&aring;", "a", "å", "å"));
        m.insert("AElig", ("\\AE{}", false, "&AElig;", "AE", "Æ", "Æ"));
        m.insert("aelig", ("\\ae{}", false, "&aelig;", "ae", "æ", "æ"));
        m.insert("Ccedil", ("\\c{C}", false, "&Ccedil;", "C", "Ç", "Ç"));
        m.insert("ccedil", ("\\c{c}", false, "&ccedil;", "c", "ç", "ç"));
        m.insert("Egrave", ("\\`{E}", false, "&Egrave;", "E", "È", "È"));
        m.insert("egrave", ("\\`{e}", false, "&egrave;", "e", "è", "è"));
        m.insert("Eacute", ("\\'{E}", false, "&Eacute;", "E", "É", "É"));
        m.insert("eacute", ("\\'{e}", false, "&eacute;", "e", "é", "é"));
        m.insert("Ecirc", ("\\^{E}", false, "&Ecirc;", "E", "Ê", "Ê"));
        m.insert("ecirc", ("\\^{e}", false, "&ecirc;", "e", "ê", "ê"));
        m.insert("Euml", ("\\\"{E}", false, "&Euml;", "E", "Ë", "Ë"));
        m.insert("euml", ("\\\"{e}", false, "&euml;", "e", "ë", "ë"));
        m.insert("Igrave", ("\\`{I}", false, "&Igrave;", "I", "Ì", "Ì"));
        m.insert("igrave", ("\\`{i}", false, "&igrave;", "i", "ì", "ì"));
        m.insert("Iacute", ("\\'{I}", false, "&Iacute;", "I", "Í", "Í"));
        m.insert("iacute", ("\\'{i}", false, "&iacute;", "i", "í", "í"));
        m.insert("Icirc", ("\\^{I}", false, "&Icirc;", "I", "Î", "Î"));
        m.insert("icirc", ("\\^{i}", false, "&icirc;", "i", "î", "î"));
        m.insert("Iuml", ("\\\"{I}", false, "&Iuml;", "I", "Ï", "Ï"));
        m.insert("iuml", ("\\\"{i}", false, "&iuml;", "i", "ï", "ï"));
        m.insert("Ntilde", ("\\~{N}", false, "&Ntilde;", "N", "Ñ", "Ñ"));
        m.insert("ntilde", ("\\~{n}", false, "&ntilde;", "n", "ñ", "ñ"));
        m.insert("Ograve", ("\\`{O}", false, "&Ograve;", "O", "Ò", "Ò"));
        m.insert("ograve", ("\\`{o}", false, "&ograve;", "o", "ò", "ò"));
        m.insert("Oacute", ("\\'{O}", false, "&Oacute;", "O", "Ó", "Ó"));
        m.insert("oacute", ("\\'{o}", false, "&oacute;", "o", "ó", "ó"));
        m.insert("Ocirc", ("\\^{O}", false, "&Ocirc;", "O", "Ô", "Ô"));
        m.insert("ocirc", ("\\^{o}", false, "&ocirc;", "o", "ô", "ô"));
        m.insert("Otilde", ("\\~{O}", false, "&Otilde;", "O", "Õ", "Õ"));
        m.insert("otilde", ("\\~{o}", false, "&otilde;", "o", "õ", "õ"));
        m.insert("Ouml", ("\\\"{O}", false, "&Ouml;", "Oe", "Ö", "Ö"));
        m.insert("ouml", ("\\\"{o}", false, "&ouml;", "oe", "ö", "ö"));
        m.insert("Oslash", ("\\O", false, "&Oslash;", "O", "Ø", "Ø"));
        m.insert("oslash", ("\\o{}", false, "&oslash;", "o", "ø", "ø"));
        m.insert("OElig", ("\\OE{}", false, "&OElig;", "OE", "OE", "Œ"));
        m.insert("oelig", ("\\oe{}", false, "&oelig;", "oe", "oe", "œ"));
        m.insert("Scaron", ("\\v{S}", false, "&Scaron;", "S", "S", "Š"));
        m.insert("scaron", ("\\v{s}", false, "&scaron;", "s", "s", "š"));
        m.insert("szlig", ("\\ss{}", false, "&szlig;", "ss", "ß", "ß"));
        m.insert("Ugrave", ("\\`{U}", false, "&Ugrave;", "U", "Ù", "Ù"));
        m.insert("ugrave", ("\\`{u}", false, "&ugrave;", "u", "ù", "ù"));
        m.insert("Uacute", ("\\'{U}", false, "&Uacute;", "U", "Ú", "Ú"));
        m.insert("uacute", ("\\'{u}", false, "&uacute;", "u", "ú", "ú"));
        m.insert("Ucirc", ("\\^{U}", false, "&Ucirc;", "U", "Û", "Û"));
        m.insert("ucirc", ("\\^{u}", false, "&ucirc;", "u", "û", "û"));
        m.insert("Uuml", ("\\\"{U}", false, "&Uuml;", "Ue", "Ü", "Ü"));
        m.insert("uuml", ("\\\"{u}", false, "&uuml;", "ue", "ü", "ü"));
        m.insert("Yacute", ("\\'{Y}", false, "&Yacute;", "Y", "Ý", "Ý"));
        m.insert("yacute", ("\\'{y}", false, "&yacute;", "y", "ý", "ý"));
        m.insert("Yuml", ("\\\"{Y}", false, "&Yuml;", "Y", "Y", "Ÿ"));
        m.insert("yuml", ("\\\"{y}", false, "&yuml;", "y", "ÿ", "ÿ"));

        // Latin (special face)
        m.insert("fnof", ("\\textit{f}", false, "&fnof;", "f", "f", "ƒ"));
        m.insert("real", ("\\Re", true, "&real;", "R", "R", "ℜ"));
        m.insert("image", ("\\Im", true, "&image;", "I", "I", "ℑ"));
        m.insert("weierp", ("\\wp", true, "&weierp;", "P", "P", "℘"));
        m.insert("ell", ("\\ell", true, "&ell;", "ell", "ell", "ℓ"));
        m.insert("imath", ("\\imath", true, "&imath;", "[dotless i]", "dotless i", "ı"));
        m.insert("jmath", ("\\jmath", true, "&jmath;", "[dotless j]", "dotless j", "ȷ"));

        // Greek
        m.insert("Alpha", ("A", false, "&Alpha;", "Alpha", "Alpha", "Α"));
        m.insert("alpha", ("\\alpha", true, "&alpha;", "alpha", "alpha", "α"));
        m.insert("Beta", ("B", false, "&Beta;", "Beta", "Beta", "Β"));
        m.insert("beta", ("\\beta", true, "&beta;", "beta", "beta", "β"));
        m.insert("Gamma", ("\\Gamma", true, "&Gamma;", "Gamma", "Gamma", "Γ"));
        m.insert("gamma", ("\\gamma", true, "&gamma;", "gamma", "gamma", "γ"));
        m.insert("Delta", ("\\Delta", true, "&Delta;", "Delta", "Delta", "Δ"));
        m.insert("delta", ("\\delta", true, "&delta;", "delta", "delta", "δ"));
        m.insert("Epsilon", ("E", false, "&Epsilon;", "Epsilon", "Epsilon", "Ε"));
        m.insert("epsilon", ("\\epsilon", true, "&epsilon;", "epsilon", "epsilon", "ε"));
        m.insert("varepsilon", ("\\varepsilon", true, "&epsilon;", "varepsilon", "varepsilon", "ε"));
        m.insert("Zeta", ("Z", false, "&Zeta;", "Zeta", "Zeta", "Ζ"));
        m.insert("zeta", ("\\zeta", true, "&zeta;", "zeta", "zeta", "ζ"));
        m.insert("Eta", ("H", false, "&Eta;", "Eta", "Eta", "Η"));
        m.insert("eta", ("\\eta", true, "&eta;", "eta", "eta", "η"));
        m.insert("Theta", ("\\Theta", true, "&Theta;", "Theta", "Theta", "Θ"));
        m.insert("theta", ("\\theta", true, "&theta;", "theta", "theta", "θ"));
        m.insert("thetasym", ("\\vartheta", true, "&thetasym;", "theta", "theta", "ϑ"));
        m.insert("vartheta", ("\\vartheta", true, "&thetasym;", "theta", "theta", "ϑ"));
        m.insert("Iota", ("I", false, "&Iota;", "Iota", "Iota", "Ι"));
        m.insert("iota", ("\\iota", true, "&iota;", "iota", "iota", "ι"));
        m.insert("Kappa", ("K", false, "&Kappa;", "Kappa", "Kappa", "Κ"));
        m.insert("kappa", ("\\kappa", true, "&kappa;", "kappa", "kappa", "κ"));
        m.insert("Lambda", ("\\Lambda", true, "&Lambda;", "Lambda", "Lambda", "Λ"));
        m.insert("lambda", ("\\lambda", true, "&lambda;", "lambda", "lambda", "λ"));
        m.insert("Mu", ("M", false, "&Mu;", "Mu", "Mu", "Μ"));
        m.insert("mu", ("\\mu", true, "&mu;", "mu", "mu", "μ"));
        m.insert("nu", ("\\nu", true, "&nu;", "nu", "nu", "ν"));
        m.insert("Nu", ("N", false, "&Nu;", "Nu", "Nu", "Ν"));
        m.insert("Xi", ("\\Xi", true, "&Xi;", "Xi", "Xi", "Ξ"));
        m.insert("xi", ("\\xi", true, "&xi;", "xi", "xi", "ξ"));
        m.insert("Omicron", ("O", false, "&Omicron;", "Omicron", "Omicron", "Ο"));
        m.insert("omicron", ("\\textit{o}", false, "&omicron;", "omicron", "omicron", "ο"));
        m.insert("Pi", ("\\Pi", true, "&Pi;", "Pi", "Pi", "Π"));
        m.insert("pi", ("\\pi", true, "&pi;", "pi", "pi", "π"));
        m.insert("Rho", ("P", false, "&Rho;", "Rho", "Rho", "Ρ"));
        m.insert("rho", ("\\rho", true, "&rho;", "rho", "rho", "ρ"));
        m.insert("Sigma", ("\\Sigma", true, "&Sigma;", "Sigma", "Sigma", "Σ"));
        m.insert("sigma", ("\\sigma", true, "&sigma;", "sigma", "sigma", "σ"));
        m.insert("sigmaf", ("\\varsigma", true, "&sigmaf;", "sigmaf", "sigmaf", "ς"));
        m.insert("varsigma", ("\\varsigma", true, "&sigmaf;", "varsigma", "varsigma", "ς"));
        m.insert("Tau", ("T", false, "&Tau;", "Tau", "Tau", "Τ"));
        m.insert("Upsilon", ("\\Upsilon", true, "&Upsilon;", "Upsilon", "Upsilon", "Υ"));
        m.insert("upsih", ("\\Upsilon", true, "&upsih;", "upsilon", "upsilon", "ϒ"));
        m.insert("upsilon", ("\\upsilon", true, "&upsilon;", "upsilon", "upsilon", "υ"));
        m.insert("Phi", ("\\Phi", true, "&Phi;", "Phi", "Phi", "Φ"));
        m.insert("phi", ("\\phi", true, "&phi;", "phi", "phi", "ɸ"));
        m.insert("varphi", ("\\varphi", true, "&varphi;", "varphi", "varphi", "φ"));
        m.insert("Chi", ("X", false, "&Chi;", "Chi", "Chi", "Χ"));
        m.insert("chi", ("\\chi", true, "&chi;", "chi", "chi", "χ"));
        m.insert("acutex", ("\\acute x", true, "&acute;x", "'x", "'x", "𝑥́"));
        m.insert("Psi", ("\\Psi", true, "&Psi;", "Psi", "Psi", "Ψ"));
        m.insert("psi", ("\\psi", true, "&psi;", "psi", "psi", "ψ"));
        m.insert("tau", ("\\tau", true, "&tau;", "tau", "tau", "τ"));
        m.insert("Omega", ("\\Omega", true, "&Omega;", "Omega", "Omega", "Ω"));
        m.insert("omega", ("\\omega", true, "&omega;", "omega", "omega", "ω"));
        m.insert("piv", ("\\varpi", true, "&piv;", "omega-pi", "omega-pi", "ϖ"));
        m.insert("varpi", ("\\varpi", true, "&piv;", "omega-pi", "omega-pi", "ϖ"));
        m.insert("partial", ("\\partial", true, "&part;", "[partial differential]", "[partial differential]", "∂"));

        // Hebrew
        m.insert("alefsym", ("\\aleph", true, "&alefsym;", "aleph", "aleph", "ℵ"));
        m.insert("aleph", ("\\aleph", true, "&aleph;", "aleph", "aleph", "ℵ"));
        m.insert("gimel", ("\\gimel", true, "&gimel;", "gimel", "gimel", "ℷ"));
        m.insert("beth", ("\\beth", true, "&beth;", "beth", "beth", "ב"));
        m.insert("dalet", ("\\daleth", true, "&daleth;", "dalet", "dalet", "ד"));

        // Dead languages
        m.insert("ETH", ("\\DH{}", false, "&ETH;", "D", "Ð", "Ð"));
        m.insert("eth", ("\\dh{}", false, "&eth;", "dh", "ð", "ð"));
        m.insert("THORN", ("\\TH{}", false, "&THORN;", "TH", "Þ", "Þ"));
        m.insert("thorn", ("\\th{}", false, "&thorn;", "th", "þ", "þ"));

        // Punctuation
        // Dots and Marks
        m.insert("dots", ("\\dots{}", false, "&hellip;", "...", "...", "…"));
        m.insert("cdots", ("\\cdots{}", true, "&ctdot;", "...", "...", "⋯"));
        m.insert("hellip", ("\\dots{}", false, "&hellip;", "...", "...", "…"));
        m.insert("middot", ("\\textperiodcentered{}", false, "&middot;", ".", "·", "·"));
        m.insert("iexcl", ("!`", false, "&iexcl;", "!", "¡", "¡"));
        m.insert("iquest", ("?`", false, "&iquest;", "?", "¿", "¿"));

        // Dash-like
        m.insert("shy", ("\\-", false, "&shy;", "", "", ""));
        m.insert("ndash", ("--", false, "&ndash;", "-", "-", "–"));
        m.insert("mdash", ("---", false, "&mdash;", "--", "--", "—"));

        // Quotations
        m.insert("quot", ("\\textquotedbl{}", false, "&quot;", "\"", "\"", "\""));
        m.insert("acute", ("\\textasciiacute{}", false, "&acute;", "'", "´", "´"));
        m.insert("ldquo", ("\\textquotedblleft{}", false, "&ldquo;", "\"", "\"", "“"));
        m.insert("rdquo", ("\\textquotedblright{}", false, "&rdquo;", "\"", "\"", "”"));
        m.insert("bdquo", ("\\quotedblbase{}", false, "&bdquo;", "\"", "\"", "„"));
        m.insert("lsquo", ("\\textquoteleft{}", false, "&lsquo;", "`", "`", "‘"));
        m.insert("rsquo", ("\\textquoteright{}", false, "&rsquo;", "'", "'", "’"));
        m.insert("sbquo", ("\\quotesinglbase{}", false, "&sbquo;", ",", ",", "‚"));
        m.insert("laquo", ("\\guillemotleft{}", false, "&laquo;", "<<", "«", "«"));
        m.insert("raquo", ("\\guillemotright{}", false, "&raquo;", ">>", "»", "»"));
        m.insert("lsaquo", ("\\guilsinglleft{}", false, "&lsaquo;", "<", "<", "‹"));
        m.insert("rsaquo", ("\\guilsinglright{}", false, "&rsaquo;", ">", ">", "›"));

        // Other
        // Misc. (often used)
        m.insert("circ", ("\\^{}", false, "&circ;", "^", "^", "∘"));
        m.insert("vert", ("\\vert{}", true, "&vert;", "|", "|", "|"));
        m.insert("vbar", ("|", false, "|", "|", "|", "|"));
        m.insert("brvbar", ("\\textbrokenbar{}", false, "&brvbar;", "|", "¦", "¦"));
        m.insert("S", ("\\S", false, "&sect;", "paragraph", "§", "§"));
        m.insert("sect", ("\\S", false, "&sect;", "paragraph", "§", "§"));
        m.insert("amp", ("\\&", false, "&amp;", "&", "&", "&"));
        m.insert("lt", ("\\textless{}", false, "&lt;", "<", "<", "<"));
        m.insert("gt", ("\\textgreater{}", false, "&gt;", ">", ">", ">"));
        m.insert("tilde", ("\\textasciitilde{}", false, "~", "~", "~", "~"));
        m.insert("slash", ("/", false, "/", "/", "/", "/"));
        m.insert("plus", ("+", false, "+", "+", "+", "+"));
        m.insert("under", ("\\_", false, "_", "_", "_", "_"));
        m.insert("equal", ("=", false, "=", "=", "=", "="));
        m.insert("asciicirc", ("\\textasciicircum{}", false, "^", "^", "^", "^"));
        m.insert("dagger", ("\\textdagger{}", false, "&dagger;", "[dagger]", "[dagger]", "†"));
        m.insert("dag", ("\\dag{}", false, "&dagger;", "[dagger]", "[dagger]", "†"));
        m.insert("Dagger", ("\\textdaggerdbl{}", false, "&Dagger;", "[doubledagger]", "[doubledagger]", "‡"));
        m.insert("ddag", ("\\ddag{}", false, "&Dagger;", "[doubledagger]", "[doubledagger]", "‡"));

        // Whitespace
        m.insert("nbsp", ("~", false, "&nbsp;", " ", "\x00A0", "\x00A0"));
        m.insert("ensp", ("\\hspace*{.5em}", false, "&ensp;", " ", " ", " "));
        m.insert("emsp", ("\\hspace*{1em}", false, "&emsp;", " ", " ", " "));
        m.insert("thinsp", ("\\hspace*{.2em}", false, "&thinsp;", " ", " ", " "));

        // Currency
        m.insert("curren", ("\\textcurrency{}", false, "&curren;", "curr.", "¤", "¤"));
        m.insert("cent", ("\\textcent{}", false, "&cent;", "cent", "¢", "¢"));
        m.insert("pound", ("\\pounds{}", false, "&pound;", "pound", "£", "£"));
        m.insert("yen", ("\\textyen{}", false, "&yen;", "yen", "¥", "¥"));
        m.insert("euro", ("\\texteuro{}", false, "&euro;", "EUR", "EUR", "€"));
        m.insert("EUR", ("\\texteuro{}", false, "&euro;", "EUR", "EUR", "€"));
        m.insert("dollar", ("\\$", false, "$", "$", "$", "$"));
        m.insert("USD", ("\\$", false, "$", "$", "$", "$"));

        // Property Marks
        m.insert("copy", ("\\textcopyright{}", false, "&copy;", "(c)", "©", "©"));
        m.insert("reg", ("\\textregistered{}", false, "&reg;", "(r)", "®", "®"));
        m.insert("trade", ("\\texttrademark{}", false, "&trade;", "TM", "TM", "™"));

        // Science et al.
        m.insert("minus", ("\\minus", true, "&minus;", "-", "-", "−"));
        m.insert("pm", ("\\textpm{}", false, "&plusmn;", "+-", "±", "±"));
        m.insert("plusmn", ("\\textpm{}", false, "&plusmn;", "+-", "±", "±"));
        m.insert("times", ("\\texttimes{}", false, "&times;", "*", "×", "×"));
        m.insert("frasl", ("/", false, "&frasl;", "/", "/", "⁄"));
        m.insert("colon", ("\\colon", true, ":", ":", ":", ":"));
        m.insert("div", ("\\textdiv{}", false, "&divide;", "/", "÷", "÷"));
        m.insert("frac12", ("\\textonehalf{}", false, "&frac12;", "1/2", "½", "½"));
        m.insert("frac14", ("\\textonequarter{}", false, "&frac14;", "1/4", "¼", "¼"));
        m.insert("frac34", ("\\textthreequarters{}", false, "&frac34;", "3/4", "¾", "¾"));
        m.insert("permil", ("\\textperthousand{}", false, "&permil;", "per thousand", "per thousand", "‰"));
        m.insert("sup1", ("\\textonesuperior{}", false, "&sup1;", "^1", "¹", "¹"));
        m.insert("sup2", ("\\texttwosuperior{}", false, "&sup2;", "^2", "²", "²"));
        m.insert("sup3", ("\\textthreesuperior{}", false, "&sup3;", "^3", "³", "³"));
        m.insert("radic", ("\\sqrt{\\,}", true, "&radic;", "[square root]", "[square root]", "√"));
        m.insert("sum", ("\\sum", true, "&sum;", "[sum]", "[sum]", "∑"));
        m.insert("prod", ("\\prod", true, "&prod;", "[product]", "[n-ary product]", "∏"));
        m.insert("micro", ("\\textmu{}", false, "&micro;", "micro", "µ", "µ"));
        m.insert("macr", ("\\textasciimacron{}", false, "&macr;", "[macron]", "¯", "¯"));
        m.insert("deg", ("\\textdegree{}", false, "&deg;", "degree", "°", "°"));
        m.insert("prime", ("\\prime", true, "&prime;", "'", "'", "′"));
        m.insert("Prime", ("\\prime{}\\prime", true, "&Prime;", "''", "''", "″"));
        m.insert("infin", ("\\infty", true, "&infin;", "[infinity]", "[infinity]", "∞"));
        m.insert("infty", ("\\infty", true, "&infin;", "[infinity]", "[infinity]", "∞"));
        m.insert("prop", ("\\propto", true, "&prop;", "[proportional to]", "[proportional to]", "∝"));
        m.insert("propto", ("\\propto", true, "&prop;", "[proportional to]", "[proportional to]", "∝"));
        m.insert("not", ("\\textlnot{}", false, "&not;", "[angled dash]", "¬", "¬"));
        m.insert("neg", ("\\neg{}", true, "&not;", "[angled dash]", "¬", "¬"));
        m.insert("land", ("\\land", true, "&and;", "[logical and]", "[logical and]", "∧"));
        m.insert("wedge", ("\\wedge", true, "&and;", "[logical and]", "[logical and]", "∧"));
        m.insert("lor", ("\\lor", true, "&or;", "[logical or]", "[logical or]", "∨"));
        m.insert("vee", ("\\vee", true, "&or;", "[logical or]", "[logical or]", "∨"));
        m.insert("cap", ("\\cap", true, "&cap;", "[intersection]", "[intersection]", "∩"));
        m.insert("cup", ("\\cup", true, "&cup;", "[union]", "[union]", "∪"));
        m.insert("smile", ("\\smile", true, "&smile;", "[cup product]", "[cup product]", "⌣"));
        m.insert("frown", ("\\frown", true, "&frown;", "[Cap product]", "[cap product]", "⌢"));
        m.insert("int", ("\\int", true, "&int;", "[integral]", "[integral]", "∫"));
        m.insert("therefore", ("\\therefore", true, "&there4;", "[therefore]", "[therefore]", "∴"));
        m.insert("there4", ("\\therefore", true, "&there4;", "[therefore]", "[therefore]", "∴"));
        m.insert("because", ("\\because", true, "&because;", "[because]", "[because]", "∵"));
        m.insert("sim", ("\\sim", true, "&sim;", "~", "~", "∼"));
        m.insert("cong", ("\\cong", true, "&cong;", "[approx. equal to]", "[approx. equal to]", "≅"));
        m.insert("simeq", ("\\simeq", true, "&cong;",  "[approx. equal to]", "[approx. equal to]", "≅"));
        m.insert("asymp", ("\\asymp", true, "&asymp;", "[almost equal to]", "[almost equal to]", "≈"));
        m.insert("approx", ("\\approx", true, "&asymp;", "[almost equal to]", "[almost equal to]", "≈"));
        m.insert("ne", ("\\ne", true, "&ne;", "[not equal to]", "[not equal to]", "≠"));
        m.insert("neq", ("\\neq", true, "&ne;", "[not equal to]", "[not equal to]", "≠"));
        m.insert("equiv", ("\\equiv", true, "&equiv;", "[identical to]", "[identical to]", "≡"));

        m.insert("triangleq", ("\\triangleq", true, "&triangleq;", "[defined to]", "[defined to]", "≜"));
        m.insert("le", ("\\le", true, "&le;", "<=", "<=", "≤"));
        m.insert("leq", ("\\le", true, "&le;", "<=", "<=", "≤"));
        m.insert("ge", ("\\ge", true, "&ge;", ">=", ">=", "≥"));
        m.insert("geq", ("\\ge", true, "&ge;", ">=", ">=", "≥"));
        m.insert("lessgtr", ("\\lessgtr", true, "&lessgtr;", "[less than or greater than]", "[less than or greater than]", "≶"));
        m.insert("lesseqgtr", ("\\lesseqgtr", true, "&lesseqgtr;", "[less than or equal or greater than or equal]", "[less than or equal or greater than or equal]", "⋚"));
        m.insert("ll", ("\\ll", true,  "&Lt;", "<<", "<<", "≪"));
        m.insert("Ll", ("\\lll", true, "&Ll;", "<<<", "<<<", "⋘"));
        m.insert("lll", ("\\lll", true, "&Ll;", "<<<", "<<<", "⋘"));
        m.insert("gg", ("\\gg", true,  "&Gt;", ">>", ">>", "≫"));
        m.insert("Gg", ("\\ggg", true, "&Gg;", ">>>", ">>>", "⋙"));
        m.insert("ggg", ("\\ggg", true, "&Gg;", ">>>", ">>>", "⋙"));
        m.insert("prec", ("\\prec", true, "&pr;", "[precedes]", "[precedes]", "≺"));
        m.insert("preceq", ("\\preceq", true, "&prcue;", "[precedes or equal]", "[precedes or equal]", "≼"));
        m.insert("preccurlyeq", ("\\preccurlyeq", true, "&prcue;", "[precedes or equal]", "[precedes or equal]", "≼"));
        m.insert("succ", ("\\succ", true, "&sc;", "[succeeds]", "[succeeds]", "≻"));
        m.insert("succeq", ("\\succeq", true, "&sccue;", "[succeeds or equal]", "[succeeds or equal]", "≽"));
        m.insert("succcurlyeq", ("\\succcurlyeq", true, "&sccue;", "[succeeds or equal]", "[succeeds or equal]", "≽"));
        m.insert("sub", ("\\subset", true, "&sub;", "[subset of]", "[subset of]", "⊂"));
        m.insert("subset", ("\\subset", true, "&sub;", "[subset of]", "[subset of]", "⊂"));
        m.insert("sup", ("\\supset", true, "&sup;", "[superset of]", "[superset of]", "⊃"));
        m.insert("supset", ("\\supset", true, "&sup;", "[superset of]", "[superset of]", "⊃"));
        m.insert("nsub", ("\\not\\subset", true, "&nsub;", "[not a subset of]", "[not a subset of", "⊄"));
        m.insert("sube", ("\\subseteq", true, "&sube;", "[subset of or equal to]", "[subset of or equal to]", "⊆"));
        m.insert("nsup", ("\\not\\supset", true, "&nsup;", "[not a superset of]", "[not a superset of]", "⊅"));
        m.insert("supe", ("\\supseteq", true, "&supe;", "[superset of or equal to]", "[superset of or equal to]", "⊇"));
        m.insert("setminus", ("\\setminus", true, "&setminus;", "\\", "\\", "⧵"));
        m.insert("forall", ("\\forall", true, "&forall;", "[for all]", "[for all]", "∀"));
        m.insert("exist", ("\\exists", true, "&exist;", "[there exists]", "[there exists]", "∃"));
        m.insert("exists", ("\\exists", true, "&exist;", "[there exists]", "[there exists]", "∃"));
        m.insert("nexist", ("\\nexists", true, "&exist;", "[there does not exists]", "[there does not  exists]", "∄"));
        m.insert("nexists", ("\\nexists", true, "&exist;", "[there does not exists]", "[there does not  exists]", "∄"));
        m.insert("empty", ("\\empty", true, "&empty;", "[empty set]", "[empty set]", "∅"));
        m.insert("emptyset", ("\\emptyset", true, "&empty;", "[empty set]", "[empty set]", "∅"));
        m.insert("isin", ("\\in", true, "&isin;", "[element of]", "[element of]", "∈"));
        m.insert("in", ("\\in", true, "&isin;", "[element of]", "[element of]", "∈"));
        m.insert("notin", ("\\notin", true, "&notin;", "[not an element of]", "[not an element of]", "∉"));
        m.insert("ni", ("\\ni", true, "&ni;", "[contains as member]", "[contains as member]", "∋"));
        m.insert("nabla", ("\\nabla", true, "&nabla;", "[nabla]", "[nabla]", "∇"));
        m.insert("ang", ("\\angle", true, "&ang;", "[angle]", "[angle]", "∠"));
        m.insert("angle", ("\\angle", true, "&ang;", "[angle]", "[angle]", "∠"));
        m.insert("perp", ("\\perp", true, "&perp;", "[up tack]", "[up tack]", "⊥"));
        m.insert("parallel", ("\\parallel", true, "&parallel;", "||", "||", "∥"));
        m.insert("sdot", ("\\cdot", true, "&sdot;", "[dot]", "[dot]", "⋅"));
        m.insert("cdot", ("\\cdot", true, "&sdot;", "[dot]", "[dot]", "⋅"));
        m.insert("lceil", ("\\lceil", true, "&lceil;", "[left ceiling]", "[left ceiling]", "⌈"));
        m.insert("rceil", ("\\rceil", true, "&rceil;", "[right ceiling]", "[right ceiling]", "⌉"));
        m.insert("lfloor", ("\\lfloor", true, "&lfloor;", "[left floor]", "[left floor]", "⌊"));
        m.insert("rfloor", ("\\rfloor", true, "&rfloor;", "[right floor]", "[right floor]", "⌋"));
        m.insert("lang", ("\\langle", true, "&lang;", "<", "<", "⟨"));
        m.insert("rang", ("\\rangle", true, "&rang;", ">", ">", "⟩"));
        m.insert("langle", ("\\langle", true, "&lang;", "<", "<", "⟨"));
        m.insert("rangle", ("\\rangle", true, "&rang;", ">", ">", "⟩"));
        m.insert("hbar", ("\\hbar", true, "&hbar;", "hbar", "hbar", "ℏ"));
        m.insert("mho", ("\\mho", true, "&mho;", "mho", "mho", "℧"));

        // Arrows
        m.insert("larr", ("\\leftarrow", true, "&larr;", "<-", "<-", "←"));
        m.insert("leftarrow", ("\\leftarrow", true, "&larr;",  "<-", "<-", "←"));
        m.insert("gets", ("\\gets", true, "&larr;",  "<-", "<-", "←"));
        m.insert("lArr", ("\\Leftarrow", true, "&lArr;", "<=", "<=", "⇐"));
        m.insert("Leftarrow", ("\\Leftarrow", true, "&lArr;", "<=", "<=", "⇐"));
        m.insert("uarr", ("\\uparrow", true, "&uarr;", "[uparrow]", "[uparrow]", "↑"));
        m.insert("uparrow", ("\\uparrow", true, "&uarr;", "[uparrow]", "[uparrow]", "↑"));
        m.insert("uArr", ("\\Uparrow", true, "&uArr;", "[dbluparrow]", "[dbluparrow]", "⇑"));
        m.insert("Uparrow", ("\\Uparrow", true, "&uArr;", "[dbluparrow]", "[dbluparrow]", "⇑"));
        m.insert("rarr", ("\\rightarrow", true, "&rarr;", "->", "->", "→"));
        m.insert("to", ("\\to", true, "&rarr;", "->", "->", "→"));
        m.insert("rightarrow", ("\\rightarrow", true, "&rarr;",  "->", "->", "→"));
        m.insert("rArr", ("\\Rightarrow", true, "&rArr;", "=>", "=>", "⇒"));
        m.insert("Rightarrow", ("\\Rightarrow", true, "&rArr;", "=>", "=>", "⇒"));
        m.insert("darr", ("\\downarrow", true, "&darr;", "[downarrow]", "[downarrow]", "↓"));
        m.insert("downarrow", ("\\downarrow", true, "&darr;", "[downarrow]", "[downarrow]", "↓"));
        m.insert("dArr", ("\\Downarrow", true, "&dArr;", "[dbldownarrow]", "[dbldownarrow]", "⇓"));
        m.insert("Downarrow", ("\\Downarrow", true, "&dArr;", "[dbldownarrow]", "[dbldownarrow]", "⇓"));
        m.insert("harr", ("\\leftrightarrow", true, "&harr;", "<->", "<->", "↔"));
        m.insert("leftrightarrow", ("\\leftrightarrow", true, "&harr;",  "<->", "<->", "↔"));
        m.insert("hArr", ("\\Leftrightarrow", true, "&hArr;", "<=>", "<=>", "⇔"));
        m.insert("Leftrightarrow", ("\\Leftrightarrow", true, "&hArr;", "<=>", "<=>", "⇔"));
        m.insert("crarr", ("\\hookleftarrow", true, "&crarr;", "<-'", "<-'", "↵"));
        m.insert("hookleftarrow", ("\\hookleftarrow", true, "&crarr;",  "<-'", "<-'", "↵"));

        // Function names
        m.insert("arccos", ("\\arccos", true, "arccos", "arccos", "arccos", "arccos"));
        m.insert("arcsin", ("\\arcsin", true, "arcsin", "arcsin", "arcsin", "arcsin"));
        m.insert("arctan", ("\\arctan", true, "arctan", "arctan", "arctan", "arctan"));
        m.insert("arg", ("\\arg", true, "arg", "arg", "arg", "arg"));
        m.insert("cos", ("\\cos", true, "cos", "cos", "cos", "cos"));
        m.insert("cosh", ("\\cosh", true, "cosh", "cosh", "cosh", "cosh"));
        m.insert("cot", ("\\cot", true, "cot", "cot", "cot", "cot"));
        m.insert("coth", ("\\coth", true, "coth", "coth", "coth", "coth"));
        m.insert("csc", ("\\csc", true, "csc", "csc", "csc", "csc"));
        m.insert("deg", ("\\deg", true, "&deg;", "deg", "deg", "deg"));
        m.insert("det", ("\\det", true, "det", "det", "det", "det"));
        m.insert("dim", ("\\dim", true, "dim", "dim", "dim", "dim"));
        m.insert("exp", ("\\exp", true, "exp", "exp", "exp", "exp"));
        m.insert("gcd", ("\\gcd", true, "gcd", "gcd", "gcd", "gcd"));
        m.insert("hom", ("\\hom", true, "hom", "hom", "hom", "hom"));
        m.insert("inf", ("\\inf", true, "inf", "inf", "inf", "inf"));
        m.insert("ker", ("\\ker", true, "ker", "ker", "ker", "ker"));
        m.insert("lg", ("\\lg", true, "lg", "lg", "lg", "lg"));
        m.insert("lim", ("\\lim", true, "lim", "lim", "lim", "lim"));
        m.insert("liminf", ("\\liminf", true, "liminf", "liminf", "liminf", "liminf"));
        m.insert("limsup", ("\\limsup", true, "limsup", "limsup", "limsup", "limsup"));
        m.insert("ln", ("\\ln", true, "ln", "ln", "ln", "ln"));
        m.insert("log", ("\\log", true, "log", "log", "log", "log"));
        m.insert("max", ("\\max", true, "max", "max", "max", "max"));
        m.insert("min", ("\\min", true, "min", "min", "min", "min"));
        m.insert("Pr", ("\\Pr", true, "Pr", "Pr", "Pr", "Pr"));
        m.insert("sec", ("\\sec", true, "sec", "sec", "sec", "sec"));
        m.insert("sin", ("\\sin", true, "sin", "sin", "sin", "sin"));
        m.insert("sinh", ("\\sinh", true, "sinh", "sinh", "sinh", "sinh"));
        m.insert("sup", ("\\sup", true, "&sup;", "sup", "sup", "sup"));
        m.insert("tan", ("\\tan", true, "tan", "tan", "tan", "tan"));
        m.insert("tanh", ("\\tanh", true, "tanh", "tanh", "tanh", "tanh"));

        // Signs & Symbols
        m.insert("bull", ("\\textbullet{}", false, "&bull;", "*", "*", "•"));
        m.insert("bullet", ("\\textbullet{}", false, "&bull;", "*", "*", "•"));
        m.insert("star", ("\\star", true, "*", "*", "*", "⋆"));
        m.insert("lowast", ("\\ast", true, "&lowast;", "*", "*", "∗"));
        m.insert("ast", ("\\ast", true, "&lowast;", "*", "*", "*"));
        m.insert("odot", ("\\odot", true, "o", "[circled dot]", "[circled dot]", "ʘ"));
        m.insert("oplus", ("\\oplus", true, "&oplus;", "[circled plus]", "[circled plus]", "⊕"));
        m.insert("otimes", ("\\otimes", true, "&otimes;", "[circled times]", "[circled times]", "⊗"));
        m.insert("check", ("\\checkmark", true, "&checkmark;", "[checkmark]", "[checkmark]", "✓"));
        m.insert("checkmark", ("\\checkmark", true, "&check;", "[checkmark]", "[checkmark]", "✓"));

        // Miscellaneous (seldom used)
        m.insert("para", ("\\P{}", false, "&para;", "[pilcrow]", "¶", "¶"));
        m.insert("ordf", ("\\textordfeminine{}", false, "&ordf;", "_a_", "ª", "ª"));
        m.insert("ordm", ("\\textordmasculine{}", false, "&ordm;", "_o_", "º", "º"));
        m.insert("cedil", ("\\c{}", false, "&cedil;", "[cedilla]", "¸", "¸"));
        m.insert("oline", ("\\overline{~}", true, "&oline;", "[overline]", "¯", "‾"));
        m.insert("uml", ("\\textasciidieresis{}", false, "&uml;", "[diaeresis]", "¨", "¨"));
        m.insert("zwnj", ("\\/{}", false, "&zwnj;", "", "", "‌"));
        m.insert("zwj", ("", false, "&zwj;", "", "", "‍"));
        m.insert("lrm", ("", false, "&lrm;", "", "", "‎"));
        m.insert("rlm", ("", false, "&rlm;", "", "", "‏"));

        // Smilies
        m.insert("smiley", ("\\ddot\\smile", true, "&#9786;", ":-)", ":-)", "☺"));
        m.insert("blacksmile", ("\\ddot\\smile", true, "&#9787;", ":-)", ":-)", "☻"));
        m.insert("sad", ("\\ddot\\frown", true, "&#9785;", ":-(", ":-(", "☹"));
        m.insert("frowny", ("\\ddot\\frown", true, "&#9785;", ":-(", ":-(", "☹"));

        // Suits
        m.insert("clubs", ("\\clubsuit", true, "&clubs;", "[clubs]", "[clubs]", "♣"));
        m.insert("clubsuit", ("\\clubsuit", true, "&clubs;", "[clubs]", "[clubs]", "♣"));
        m.insert("spades", ("\\spadesuit", true, "&spades;", "[spades]", "[spades]", "♠"));
        m.insert("spadesuit", ("\\spadesuit", true, "&spades;", "[spades]", "[spades]", "♠"));
        m.insert("hearts", ("\\heartsuit", true, "&hearts;", "[hearts]", "[hearts]", "♥"));
        m.insert("heartsuit", ("\\heartsuit", true, "&heartsuit;", "[hearts]", "[hearts]", "♥"));
        m.insert("diams", ("\\diamondsuit", true, "&diams;", "[diamonds]", "[diamonds]", "◆"));
        m.insert("diamondsuit", ("\\diamondsuit", true, "&diams;", "[diamonds]", "[diamonds]", "◆"));
        m.insert("diamond", ("\\diamondsuit", true, "&diamond;", "[diamond]", "[diamond]", "◆"));
        m.insert("Diamond", ("\\diamondsuit", true, "&diamond;", "[diamond]", "[diamond]", "◆"));
        m.insert("loz", ("\\lozenge", true, "&loz;", "[lozenge]", "[lozenge]", "⧫"));

        // Spaces ("\_ ")
        // (let (space-entities html-spaces (entity "_"))
        //   (dolist (n (number-sequence 1 20) (nreverse space-entities))
        //     (let ((spaces (make-string n ?\s)))
        //   (push (list (setq entity (concat entity " "))
        //         (format "\\hspace*{%sem}" (* n .5))
        //         nil
        //         (setq html-spaces (concat "&ensp;" html-spaces))
        //         spaces
        //         spaces
        //         (make-string n ?\x2002))
        //       space-entities))))

        EntityLookupManager(m)
    };
}
