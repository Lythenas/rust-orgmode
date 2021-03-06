use phf;

/// Contains replacements for an entity in LaTeX, HTML, ASCII, Latin1 and UTF-8.
pub struct EntityReplacement {
    pub latex: &'static str,
    pub requires_latex_math: bool,
    pub html: &'static str,
    pub ascii: &'static str,
    pub latin1: &'static str,
    pub utf8: &'static str,
}

const fn make(
    latex: &'static str,
    requires_latex_math: bool,
    html: &'static str,
    ascii: &'static str,
    latin1: &'static str,
    utf8: &'static str,
) -> EntityReplacement {
    EntityReplacement {
        latex,
        requires_latex_math,
        html,
        ascii,
        latin1,
        utf8,
    }
}

/// This is a map of entity names to their replacement in the LaTeX, HTML, ASCII, Latin1 and UTF-8
/// exporters.
///
/// This list is taken from lisp/org-entities.el in the org-mode repository.
pub static ORG_ENTITIES: phf::Map<
    &'static str,
    EntityReplacement,
    //(
    //    &'static str,
    //    bool,
    //    &'static str,
    //    &'static str,
    //    &'static str,
    //    &'static str,
    //),
> = phf_map! {
    // name => LaTeX, requires LaTeX math?, html, ascii, latin1, utf-8

    // Letters
    // Latin
    "Agrave" => make("\\`{A}", false, "&Agrave;", "A", "À", "À"),
    "agrave" => make("\\`{a}", false, "&agrave;", "a", "à", "à"),
    "Aacute" => make("\\'{A}", false, "&Aacute;", "A", "Á", "Á"),
    "aacute" => make("\\'{a}", false, "&aacute;", "a", "á", "á"),
    "Acirc" => make("\\^{A}", false, "&Acirc;", "A", "Â", "Â"),
    "acirc" => make("\\^{a}", false, "&acirc;", "a", "â", "â"),
    "Amacr" => make("\\bar{A}", false, "&Amacr;", "A", "Ã", "Ã"),
    "amacr" => make("\\bar{a}", false, "&amacr;", "a", "ã", "ã"),
    "Atilde" => make("\\~{A}", false, "&Atilde;", "A", "Ã", "Ã"),
    "atilde" => make("\\~{a}", false, "&atilde;", "a", "ã", "ã"),
    "Auml" => make("\\\"{A}", false, "&Auml;", "Ae", "Ä", "Ä"),
    "auml" => make("\\\"{a}", false, "&auml;", "ae", "ä", "ä"),
    "Aring" => make("\\AA{}", false, "&Aring;", "A", "Å", "Å"),
    "AA" => make("\\AA{}", false, "&Aring;", "A", "Å", "Å"),
    "aring" => make("\\aa{}", false, "&aring;", "a", "å", "å"),
    "AElig" => make("\\AE{}", false, "&AElig;", "AE", "Æ", "Æ"),
    "aelig" => make("\\ae{}", false, "&aelig;", "ae", "æ", "æ"),
    "Ccedil" => make("\\c{C}", false, "&Ccedil;", "C", "Ç", "Ç"),
    "ccedil" => make("\\c{c}", false, "&ccedil;", "c", "ç", "ç"),
    "Egrave" => make("\\`{E}", false, "&Egrave;", "E", "È", "È"),
    "egrave" => make("\\`{e}", false, "&egrave;", "e", "è", "è"),
    "Eacute" => make("\\'{E}", false, "&Eacute;", "E", "É", "É"),
    "eacute" => make("\\'{e}", false, "&eacute;", "e", "é", "é"),
    "Ecirc" => make("\\^{E}", false, "&Ecirc;", "E", "Ê", "Ê"),
    "ecirc" => make("\\^{e}", false, "&ecirc;", "e", "ê", "ê"),
    "Euml" => make("\\\"{E}", false, "&Euml;", "E", "Ë", "Ë"),
    "euml" => make("\\\"{e}", false, "&euml;", "e", "ë", "ë"),
    "Igrave" => make("\\`{I}", false, "&Igrave;", "I", "Ì", "Ì"),
    "igrave" => make("\\`{i}", false, "&igrave;", "i", "ì", "ì"),
    "Iacute" => make("\\'{I}", false, "&Iacute;", "I", "Í", "Í"),
    "iacute" => make("\\'{i}", false, "&iacute;", "i", "í", "í"),
    "Icirc" => make("\\^{I}", false, "&Icirc;", "I", "Î", "Î"),
    "icirc" => make("\\^{i}", false, "&icirc;", "i", "î", "î"),
    "Iuml" => make("\\\"{I}", false, "&Iuml;", "I", "Ï", "Ï"),
    "iuml" => make("\\\"{i}", false, "&iuml;", "i", "ï", "ï"),
    "Ntilde" => make("\\~{N}", false, "&Ntilde;", "N", "Ñ", "Ñ"),
    "ntilde" => make("\\~{n}", false, "&ntilde;", "n", "ñ", "ñ"),
    "Ograve" => make("\\`{O}", false, "&Ograve;", "O", "Ò", "Ò"),
    "ograve" => make("\\`{o}", false, "&ograve;", "o", "ò", "ò"),
    "Oacute" => make("\\'{O}", false, "&Oacute;", "O", "Ó", "Ó"),
    "oacute" => make("\\'{o}", false, "&oacute;", "o", "ó", "ó"),
    "Ocirc" => make("\\^{O}", false, "&Ocirc;", "O", "Ô", "Ô"),
    "ocirc" => make("\\^{o}", false, "&ocirc;", "o", "ô", "ô"),
    "Otilde" => make("\\~{O}", false, "&Otilde;", "O", "Õ", "Õ"),
    "otilde" => make("\\~{o}", false, "&otilde;", "o", "õ", "õ"),
    "Ouml" => make("\\\"{O}", false, "&Ouml;", "Oe", "Ö", "Ö"),
    "ouml" => make("\\\"{o}", false, "&ouml;", "oe", "ö", "ö"),
    "Oslash" => make("\\O", false, "&Oslash;", "O", "Ø", "Ø"),
    "oslash" => make("\\o{}", false, "&oslash;", "o", "ø", "ø"),
    "OElig" => make("\\OE{}", false, "&OElig;", "OE", "OE", "Œ"),
    "oelig" => make("\\oe{}", false, "&oelig;", "oe", "oe", "œ"),
    "Scaron" => make("\\v{S}", false, "&Scaron;", "S", "S", "Š"),
    "scaron" => make("\\v{s}", false, "&scaron;", "s", "s", "š"),
    "szlig" => make("\\ss{}", false, "&szlig;", "ss", "ß", "ß"),
    "Ugrave" => make("\\`{U}", false, "&Ugrave;", "U", "Ù", "Ù"),
    "ugrave" => make("\\`{u}", false, "&ugrave;", "u", "ù", "ù"),
    "Uacute" => make("\\'{U}", false, "&Uacute;", "U", "Ú", "Ú"),
    "uacute" => make("\\'{u}", false, "&uacute;", "u", "ú", "ú"),
    "Ucirc" => make("\\^{U}", false, "&Ucirc;", "U", "Û", "Û"),
    "ucirc" => make("\\^{u}", false, "&ucirc;", "u", "û", "û"),
    "Uuml" => make("\\\"{U}", false, "&Uuml;", "Ue", "Ü", "Ü"),
    "uuml" => make("\\\"{u}", false, "&uuml;", "ue", "ü", "ü"),
    "Yacute" => make("\\'{Y}", false, "&Yacute;", "Y", "Ý", "Ý"),
    "yacute" => make("\\'{y}", false, "&yacute;", "y", "ý", "ý"),
    "Yuml" => make("\\\"{Y}", false, "&Yuml;", "Y", "Y", "Ÿ"),
    "yuml" => make("\\\"{y}", false, "&yuml;", "y", "ÿ", "ÿ"),

    // Latin (special face)
    "fnof" => make("\\textit{f}", false, "&fnof;", "f", "f", "ƒ"),
    "real" => make("\\Re", true, "&real;", "R", "R", "ℜ"),
    "image" => make("\\Im", true, "&image;", "I", "I", "ℑ"),
    "weierp" => make("\\wp", true, "&weierp;", "P", "P", "℘"),
    "ell" => make("\\ell", true, "&ell;", "ell", "ell", "ℓ"),
    "imath" => make("\\imath", true, "&imath;", "[dotless i]", "dotless i", "ı"),
    "jmath" => make("\\jmath", true, "&jmath;", "[dotless j]", "dotless j", "ȷ"),

    // Greek
    "Alpha" => make("A", false, "&Alpha;", "Alpha", "Alpha", "Α"),
    "alpha" => make("\\alpha", true, "&alpha;", "alpha", "alpha", "α"),
    "Beta" => make("B", false, "&Beta;", "Beta", "Beta", "Β"),
    "beta" => make("\\beta", true, "&beta;", "beta", "beta", "β"),
    "Gamma" => make("\\Gamma", true, "&Gamma;", "Gamma", "Gamma", "Γ"),
    "gamma" => make("\\gamma", true, "&gamma;", "gamma", "gamma", "γ"),
    "Delta" => make("\\Delta", true, "&Delta;", "Delta", "Delta", "Δ"),
    "delta" => make("\\delta", true, "&delta;", "delta", "delta", "δ"),
    "Epsilon" => make("E", false, "&Epsilon;", "Epsilon", "Epsilon", "Ε"),
    "epsilon" => make("\\epsilon", true, "&epsilon;", "epsilon", "epsilon", "ε"),
    "varepsilon" => make("\\varepsilon", true, "&epsilon;", "varepsilon", "varepsilon", "ε"),
    "Zeta" => make("Z", false, "&Zeta;", "Zeta", "Zeta", "Ζ"),
    "zeta" => make("\\zeta", true, "&zeta;", "zeta", "zeta", "ζ"),
    "Eta" => make("H", false, "&Eta;", "Eta", "Eta", "Η"),
    "eta" => make("\\eta", true, "&eta;", "eta", "eta", "η"),
    "Theta" => make("\\Theta", true, "&Theta;", "Theta", "Theta", "Θ"),
    "theta" => make("\\theta", true, "&theta;", "theta", "theta", "θ"),
    "thetasym" => make("\\vartheta", true, "&thetasym;", "theta", "theta", "ϑ"),
    "vartheta" => make("\\vartheta", true, "&thetasym;", "theta", "theta", "ϑ"),
    "Iota" => make("I", false, "&Iota;", "Iota", "Iota", "Ι"),
    "iota" => make("\\iota", true, "&iota;", "iota", "iota", "ι"),
    "Kappa" => make("K", false, "&Kappa;", "Kappa", "Kappa", "Κ"),
    "kappa" => make("\\kappa", true, "&kappa;", "kappa", "kappa", "κ"),
    "Lambda" => make("\\Lambda", true, "&Lambda;", "Lambda", "Lambda", "Λ"),
    "lambda" => make("\\lambda", true, "&lambda;", "lambda", "lambda", "λ"),
    "Mu" => make("M", false, "&Mu;", "Mu", "Mu", "Μ"),
    "mu" => make("\\mu", true, "&mu;", "mu", "mu", "μ"),
    "nu" => make("\\nu", true, "&nu;", "nu", "nu", "ν"),
    "Nu" => make("N", false, "&Nu;", "Nu", "Nu", "Ν"),
    "Xi" => make("\\Xi", true, "&Xi;", "Xi", "Xi", "Ξ"),
    "xi" => make("\\xi", true, "&xi;", "xi", "xi", "ξ"),
    "Omicron" => make("O", false, "&Omicron;", "Omicron", "Omicron", "Ο"),
    "omicron" => make("\\textit{o}", false, "&omicron;", "omicron", "omicron", "ο"),
    "Pi" => make("\\Pi", true, "&Pi;", "Pi", "Pi", "Π"),
    "pi" => make("\\pi", true, "&pi;", "pi", "pi", "π"),
    "Rho" => make("P", false, "&Rho;", "Rho", "Rho", "Ρ"),
    "rho" => make("\\rho", true, "&rho;", "rho", "rho", "ρ"),
    "Sigma" => make("\\Sigma", true, "&Sigma;", "Sigma", "Sigma", "Σ"),
    "sigma" => make("\\sigma", true, "&sigma;", "sigma", "sigma", "σ"),
    "sigmaf" => make("\\varsigma", true, "&sigmaf;", "sigmaf", "sigmaf", "ς"),
    "varsigma" => make("\\varsigma", true, "&sigmaf;", "varsigma", "varsigma", "ς"),
    "Tau" => make("T", false, "&Tau;", "Tau", "Tau", "Τ"),
    "Upsilon" => make("\\Upsilon", true, "&Upsilon;", "Upsilon", "Upsilon", "Υ"),
    "upsih" => make("\\Upsilon", true, "&upsih;", "upsilon", "upsilon", "ϒ"),
    "upsilon" => make("\\upsilon", true, "&upsilon;", "upsilon", "upsilon", "υ"),
    "Phi" => make("\\Phi", true, "&Phi;", "Phi", "Phi", "Φ"),
    "phi" => make("\\phi", true, "&phi;", "phi", "phi", "ɸ"),
    "varphi" => make("\\varphi", true, "&varphi;", "varphi", "varphi", "φ"),
    "Chi" => make("X", false, "&Chi;", "Chi", "Chi", "Χ"),
    "chi" => make("\\chi", true, "&chi;", "chi", "chi", "χ"),
    "acutex" => make("\\acute x", true, "&acute;x", "'x", "'x", "𝑥́"),
    "Psi" => make("\\Psi", true, "&Psi;", "Psi", "Psi", "Ψ"),
    "psi" => make("\\psi", true, "&psi;", "psi", "psi", "ψ"),
    "tau" => make("\\tau", true, "&tau;", "tau", "tau", "τ"),
    "Omega" => make("\\Omega", true, "&Omega;", "Omega", "Omega", "Ω"),
    "omega" => make("\\omega", true, "&omega;", "omega", "omega", "ω"),
    "piv" => make("\\varpi", true, "&piv;", "omega-pi", "omega-pi", "ϖ"),
    "varpi" => make("\\varpi", true, "&piv;", "omega-pi", "omega-pi", "ϖ"),
    "partial" => make("\\partial", true, "&part;", "[partial differential]", "[partial differential]", "∂"),

    // Hebrew
    "alefsym" => make("\\aleph", true, "&alefsym;", "aleph", "aleph", "ℵ"),
    "aleph" => make("\\aleph", true, "&aleph;", "aleph", "aleph", "ℵ"),
    "gimel" => make("\\gimel", true, "&gimel;", "gimel", "gimel", "ℷ"),
    "beth" => make("\\beth", true, "&beth;", "beth", "beth", "ב"),
    "dalet" => make("\\daleth", true, "&daleth;", "dalet", "dalet", "ד"),

    // Dead languages
    "ETH" => make("\\DH{}", false, "&ETH;", "D", "Ð", "Ð"),
    "eth" => make("\\dh{}", false, "&eth;", "dh", "ð", "ð"),
    "THORN" => make("\\TH{}", false, "&THORN;", "TH", "Þ", "Þ"),
    "thorn" => make("\\th{}", false, "&thorn;", "th", "þ", "þ"),

    // Punctuation
    // Dots and Marks
    "dots" => make("\\dots{}", false, "&hellip;", "...", "...", "…"),
    "cdots" => make("\\cdots{}", true, "&ctdot;", "...", "...", "⋯"),
    "hellip" => make("\\dots{}", false, "&hellip;", "...", "...", "…"),
    "middot" => make("\\textperiodcentered{}", false, "&middot;", ".", "·", "·"),
    "iexcl" => make("!`", false, "&iexcl;", "!", "¡", "¡"),
    "iquest" => make("?`", false, "&iquest;", "?", "¿", "¿"),

    // Dash-like
    "shy" => make("\\-", false, "&shy;", "", "", ""),
    "ndash" => make("--", false, "&ndash;", "-", "-", "–"),
    "mdash" => make("---", false, "&mdash;", "--", "--", "—"),

    // Quotations
    "quot" => make("\\textquotedbl{}", false, "&quot;", "\"", "\"", "\""),
    "acute" => make("\\textasciiacute{}", false, "&acute;", "'", "´", "´"),
    "ldquo" => make("\\textquotedblleft{}", false, "&ldquo;", "\"", "\"", "“"),
    "rdquo" => make("\\textquotedblright{}", false, "&rdquo;", "\"", "\"", "”"),
    "bdquo" => make("\\quotedblbase{}", false, "&bdquo;", "\"", "\"", "„"),
    "lsquo" => make("\\textquoteleft{}", false, "&lsquo;", "`", "`", "‘"),
    "rsquo" => make("\\textquoteright{}", false, "&rsquo;", "'", "'", "’"),
    "sbquo" => make("\\quotesinglbase{}", false, "&sbquo;", ",", ",", "‚"),
    "laquo" => make("\\guillemotleft{}", false, "&laquo;", "<<", "«", "«"),
    "raquo" => make("\\guillemotright{}", false, "&raquo;", ">>", "»", "»"),
    "lsaquo" => make("\\guilsinglleft{}", false, "&lsaquo;", "<", "<", "‹"),
    "rsaquo" => make("\\guilsinglright{}", false, "&rsaquo;", ">", ">", "›"),

    // Other
    // Misc. (often used)
    "circ" => make("\\^{}", false, "&circ;", "^", "^", "∘"),
    "vert" => make("\\vert{}", true, "&vert;", "|", "|", "|"),
    "vbar" => make("|", false, "|", "|", "|", "|"),
    "brvbar" => make("\\textbrokenbar{}", false, "&brvbar;", "|", "¦", "¦"),
    "S" => make("\\S", false, "&sect;", "paragraph", "§", "§"),
    "sect" => make("\\S", false, "&sect;", "paragraph", "§", "§"),
    "amp" => make("\\&", false, "&amp;", "&", "&", "&"),
    "lt" => make("\\textless{}", false, "&lt;", "<", "<", "<"),
    "gt" => make("\\textgreater{}", false, "&gt;", ">", ">", ">"),
    "tilde" => make("\\textasciitilde{}", false, "~", "~", "~", "~"),
    "slash" => make("/", false, "/", "/", "/", "/"),
    "plus" => make("+", false, "+", "+", "+", "+"),
    "under" => make("\\_", false, "_", "_", "_", "_"),
    "equal" => make("=", false, "=", "=", "=", "="),
    "asciicirc" => make("\\textasciicircum{}", false, "^", "^", "^", "^"),
    "dagger" => make("\\textdagger{}", false, "&dagger;", "[dagger]", "[dagger]", "†"),
    "dag" => make("\\dag{}", false, "&dagger;", "[dagger]", "[dagger]", "†"),
    "Dagger" => make("\\textdaggerdbl{}", false, "&Dagger;", "[doubledagger]", "[doubledagger]", "‡"),
    "ddag" => make("\\ddag{}", false, "&Dagger;", "[doubledagger]", "[doubledagger]", "‡"),

    // Whitespace
    "nbsp" => make("~", false, "&nbsp;", " ", "\x00A0", "\x00A0"),
    "ensp" => make("\\hspace*{.5em}", false, "&ensp;", " ", " ", " "),
    "emsp" => make("\\hspace*{1em}", false, "&emsp;", " ", " ", " "),
    "thinsp" => make("\\hspace*{.2em}", false, "&thinsp;", " ", " ", " "),

    // Currency
    "curren" => make("\\textcurrency{}", false, "&curren;", "curr.", "¤", "¤"),
    "cent" => make("\\textcent{}", false, "&cent;", "cent", "¢", "¢"),
    "pound" => make("\\pounds{}", false, "&pound;", "pound", "£", "£"),
    "yen" => make("\\textyen{}", false, "&yen;", "yen", "¥", "¥"),
    "euro" => make("\\texteuro{}", false, "&euro;", "EUR", "EUR", "€"),
    "EUR" => make("\\texteuro{}", false, "&euro;", "EUR", "EUR", "€"),
    "dollar" => make("\\$", false, "$", "$", "$", "$"),
    "USD" => make("\\$", false, "$", "$", "$", "$"),

    // Property Marks
    "copy" => make("\\textcopyright{}", false, "&copy;", "(c)", "©", "©"),
    "reg" => make("\\textregistered{}", false, "&reg;", "(r)", "®", "®"),
    "trade" => make("\\texttrademark{}", false, "&trade;", "TM", "TM", "™"),

    // Science et al.
    "minus" => make("\\minus", true, "&minus;", "-", "-", "−"),
    "pm" => make("\\textpm{}", false, "&plusmn;", "+-", "±", "±"),
    "plusmn" => make("\\textpm{}", false, "&plusmn;", "+-", "±", "±"),
    "times" => make("\\texttimes{}", false, "&times;", "*", "×", "×"),
    "frasl" => make("/", false, "&frasl;", "/", "/", "⁄"),
    "colon" => make("\\colon", true, ":", ":", ":", ":"),
    "div" => make("\\textdiv{}", false, "&divide;", "/", "÷", "÷"),
    "frac12" => make("\\textonehalf{}", false, "&frac12;", "1/2", "½", "½"),
    "frac14" => make("\\textonequarter{}", false, "&frac14;", "1/4", "¼", "¼"),
    "frac34" => make("\\textthreequarters{}", false, "&frac34;", "3/4", "¾", "¾"),
    "permil" => make("\\textperthousand{}", false, "&permil;", "per thousand", "per thousand", "‰"),
    "sup1" => make("\\textonesuperior{}", false, "&sup1;", "^1", "¹", "¹"),
    "sup2" => make("\\texttwosuperior{}", false, "&sup2;", "^2", "²", "²"),
    "sup3" => make("\\textthreesuperior{}", false, "&sup3;", "^3", "³", "³"),
    "radic" => make("\\sqrt{\\,}", true, "&radic;", "[square root]", "[square root]", "√"),
    "sum" => make("\\sum", true, "&sum;", "[sum]", "[sum]", "∑"),
    "prod" => make("\\prod", true, "&prod;", "[product]", "[n-ary product]", "∏"),
    "micro" => make("\\textmu{}", false, "&micro;", "micro", "µ", "µ"),
    "macr" => make("\\textasciimacron{}", false, "&macr;", "[macron]", "¯", "¯"),
    "deg" => make("\\textdegree{}", false, "&deg;", "degree", "°", "°"),
    "prime" => make("\\prime", true, "&prime;", "'", "'", "′"),
    "Prime" => make("\\prime{}\\prime", true, "&Prime;", "''", "''", "″"),
    "infin" => make("\\infty", true, "&infin;", "[infinity]", "[infinity]", "∞"),
    "infty" => make("\\infty", true, "&infin;", "[infinity]", "[infinity]", "∞"),
    "prop" => make("\\propto", true, "&prop;", "[proportional to]", "[proportional to]", "∝"),
    "propto" => make("\\propto", true, "&prop;", "[proportional to]", "[proportional to]", "∝"),
    "not" => make("\\textlnot{}", false, "&not;", "[angled dash]", "¬", "¬"),
    "neg" => make("\\neg{}", true, "&not;", "[angled dash]", "¬", "¬"),
    "land" => make("\\land", true, "&and;", "[logical and]", "[logical and]", "∧"),
    "wedge" => make("\\wedge", true, "&and;", "[logical and]", "[logical and]", "∧"),
    "lor" => make("\\lor", true, "&or;", "[logical or]", "[logical or]", "∨"),
    "vee" => make("\\vee", true, "&or;", "[logical or]", "[logical or]", "∨"),
    "cap" => make("\\cap", true, "&cap;", "[intersection]", "[intersection]", "∩"),
    "cup" => make("\\cup", true, "&cup;", "[union]", "[union]", "∪"),
    "smile" => make("\\smile", true, "&smile;", "[cup product]", "[cup product]", "⌣"),
    "frown" => make("\\frown", true, "&frown;", "[Cap product]", "[cap product]", "⌢"),
    "int" => make("\\int", true, "&int;", "[integral]", "[integral]", "∫"),
    "therefore" => make("\\therefore", true, "&there4;", "[therefore]", "[therefore]", "∴"),
    "there4" => make("\\therefore", true, "&there4;", "[therefore]", "[therefore]", "∴"),
    "because" => make("\\because", true, "&because;", "[because]", "[because]", "∵"),
    "sim" => make("\\sim", true, "&sim;", "~", "~", "∼"),
    "cong" => make("\\cong", true, "&cong;", "[approx. equal to]", "[approx. equal to]", "≅"),
    "simeq" => make("\\simeq", true, "&cong;",  "[approx. equal to]", "[approx. equal to]", "≅"),
    "asymp" => make("\\asymp", true, "&asymp;", "[almost equal to]", "[almost equal to]", "≈"),
    "approx" => make("\\approx", true, "&asymp;", "[almost equal to]", "[almost equal to]", "≈"),
    "ne" => make("\\ne", true, "&ne;", "[not equal to]", "[not equal to]", "≠"),
    "neq" => make("\\neq", true, "&ne;", "[not equal to]", "[not equal to]", "≠"),
    "equiv" => make("\\equiv", true, "&equiv;", "[identical to]", "[identical to]", "≡"),

    "triangleq" => make("\\triangleq", true, "&triangleq;", "[defined to]", "[defined to]", "≜"),
    "le" => make("\\le", true, "&le;", "<=", "<=", "≤"),
    "leq" => make("\\le", true, "&le;", "<=", "<=", "≤"),
    "ge" => make("\\ge", true, "&ge;", ">=", ">=", "≥"),
    "geq" => make("\\ge", true, "&ge;", ">=", ">=", "≥"),
    "lessgtr" => make("\\lessgtr", true, "&lessgtr;", "[less than or greater than]", "[less than or greater than]", "≶"),
    "lesseqgtr" => make("\\lesseqgtr", true, "&lesseqgtr;", "[less than or equal or greater than or equal]", "[less than or equal or greater than or equal]", "⋚"),
    "ll" => make("\\ll", true,  "&Lt;", "<<", "<<", "≪"),
    "Ll" => make("\\lll", true, "&Ll;", "<<<", "<<<", "⋘"),
    "lll" => make("\\lll", true, "&Ll;", "<<<", "<<<", "⋘"),
    "gg" => make("\\gg", true,  "&Gt;", ">>", ">>", "≫"),
    "Gg" => make("\\ggg", true, "&Gg;", ">>>", ">>>", "⋙"),
    "ggg" => make("\\ggg", true, "&Gg;", ">>>", ">>>", "⋙"),
    "prec" => make("\\prec", true, "&pr;", "[precedes]", "[precedes]", "≺"),
    "preceq" => make("\\preceq", true, "&prcue;", "[precedes or equal]", "[precedes or equal]", "≼"),
    "preccurlyeq" => make("\\preccurlyeq", true, "&prcue;", "[precedes or equal]", "[precedes or equal]", "≼"),
    "succ" => make("\\succ", true, "&sc;", "[succeeds]", "[succeeds]", "≻"),
    "succeq" => make("\\succeq", true, "&sccue;", "[succeeds or equal]", "[succeeds or equal]", "≽"),
    "succcurlyeq" => make("\\succcurlyeq", true, "&sccue;", "[succeeds or equal]", "[succeeds or equal]", "≽"),
    "sub" => make("\\subset", true, "&sub;", "[subset of]", "[subset of]", "⊂"),
    "subset" => make("\\subset", true, "&sub;", "[subset of]", "[subset of]", "⊂"),
    "sup" => make("\\supset", true, "&sup;", "[superset of]", "[superset of]", "⊃"),
    "supset" => make("\\supset", true, "&sup;", "[superset of]", "[superset of]", "⊃"),
    "nsub" => make("\\not\\subset", true, "&nsub;", "[not a subset of]", "[not a subset of", "⊄"),
    "sube" => make("\\subseteq", true, "&sube;", "[subset of or equal to]", "[subset of or equal to]", "⊆"),
    "nsup" => make("\\not\\supset", true, "&nsup;", "[not a superset of]", "[not a superset of]", "⊅"),
    "supe" => make("\\supseteq", true, "&supe;", "[superset of or equal to]", "[superset of or equal to]", "⊇"),
    "setminus" => make("\\setminus", true, "&setminus;", "\\", "\\", "⧵"),
    "forall" => make("\\forall", true, "&forall;", "[for all]", "[for all]", "∀"),
    "exist" => make("\\exists", true, "&exist;", "[there exists]", "[there exists]", "∃"),
    "exists" => make("\\exists", true, "&exist;", "[there exists]", "[there exists]", "∃"),
    "nexist" => make("\\nexists", true, "&exist;", "[there does not exists]", "[there does not  exists]", "∄"),
    "nexists" => make("\\nexists", true, "&exist;", "[there does not exists]", "[there does not  exists]", "∄"),
    "empty" => make("\\empty", true, "&empty;", "[empty set]", "[empty set]", "∅"),
    "emptyset" => make("\\emptyset", true, "&empty;", "[empty set]", "[empty set]", "∅"),
    "isin" => make("\\in", true, "&isin;", "[element of]", "[element of]", "∈"),
    "in" => make("\\in", true, "&isin;", "[element of]", "[element of]", "∈"),
    "notin" => make("\\notin", true, "&notin;", "[not an element of]", "[not an element of]", "∉"),
    "ni" => make("\\ni", true, "&ni;", "[contains as member]", "[contains as member]", "∋"),
    "nabla" => make("\\nabla", true, "&nabla;", "[nabla]", "[nabla]", "∇"),
    "ang" => make("\\angle", true, "&ang;", "[angle]", "[angle]", "∠"),
    "angle" => make("\\angle", true, "&ang;", "[angle]", "[angle]", "∠"),
    "perp" => make("\\perp", true, "&perp;", "[up tack]", "[up tack]", "⊥"),
    "parallel" => make("\\parallel", true, "&parallel;", "||", "||", "∥"),
    "sdot" => make("\\cdot", true, "&sdot;", "[dot]", "[dot]", "⋅"),
    "cdot" => make("\\cdot", true, "&sdot;", "[dot]", "[dot]", "⋅"),
    "lceil" => make("\\lceil", true, "&lceil;", "[left ceiling]", "[left ceiling]", "⌈"),
    "rceil" => make("\\rceil", true, "&rceil;", "[right ceiling]", "[right ceiling]", "⌉"),
    "lfloor" => make("\\lfloor", true, "&lfloor;", "[left floor]", "[left floor]", "⌊"),
    "rfloor" => make("\\rfloor", true, "&rfloor;", "[right floor]", "[right floor]", "⌋"),
    "lang" => make("\\langle", true, "&lang;", "<", "<", "⟨"),
    "rang" => make("\\rangle", true, "&rang;", ">", ">", "⟩"),
    "langle" => make("\\langle", true, "&lang;", "<", "<", "⟨"),
    "rangle" => make("\\rangle", true, "&rang;", ">", ">", "⟩"),
    "hbar" => make("\\hbar", true, "&hbar;", "hbar", "hbar", "ℏ"),
    "mho" => make("\\mho", true, "&mho;", "mho", "mho", "℧"),

    // Arrows
    "larr" => make("\\leftarrow", true, "&larr;", "<-", "<-", "←"),
    "leftarrow" => make("\\leftarrow", true, "&larr;",  "<-", "<-", "←"),
    "gets" => make("\\gets", true, "&larr;",  "<-", "<-", "←"),
    "lArr" => make("\\Leftarrow", true, "&lArr;", "<=", "<=", "⇐"),
    "Leftarrow" => make("\\Leftarrow", true, "&lArr;", "<=", "<=", "⇐"),
    "uarr" => make("\\uparrow", true, "&uarr;", "[uparrow]", "[uparrow]", "↑"),
    "uparrow" => make("\\uparrow", true, "&uarr;", "[uparrow]", "[uparrow]", "↑"),
    "uArr" => make("\\Uparrow", true, "&uArr;", "[dbluparrow]", "[dbluparrow]", "⇑"),
    "Uparrow" => make("\\Uparrow", true, "&uArr;", "[dbluparrow]", "[dbluparrow]", "⇑"),
    "rarr" => make("\\rightarrow", true, "&rarr;", "->", "->", "→"),
    "to" => make("\\to", true, "&rarr;", "->", "->", "→"),
    "rightarrow" => make("\\rightarrow", true, "&rarr;",  "->", "->", "→"),
    "rArr" => make("\\Rightarrow", true, "&rArr;", "=>", "=>", "⇒"),
    "Rightarrow" => make("\\Rightarrow", true, "&rArr;", "=>", "=>", "⇒"),
    "darr" => make("\\downarrow", true, "&darr;", "[downarrow]", "[downarrow]", "↓"),
    "downarrow" => make("\\downarrow", true, "&darr;", "[downarrow]", "[downarrow]", "↓"),
    "dArr" => make("\\Downarrow", true, "&dArr;", "[dbldownarrow]", "[dbldownarrow]", "⇓"),
    "Downarrow" => make("\\Downarrow", true, "&dArr;", "[dbldownarrow]", "[dbldownarrow]", "⇓"),
    "harr" => make("\\leftrightarrow", true, "&harr;", "<->", "<->", "↔"),
    "leftrightarrow" => make("\\leftrightarrow", true, "&harr;",  "<->", "<->", "↔"),
    "hArr" => make("\\Leftrightarrow", true, "&hArr;", "<=>", "<=>", "⇔"),
    "Leftrightarrow" => make("\\Leftrightarrow", true, "&hArr;", "<=>", "<=>", "⇔"),
    "crarr" => make("\\hookleftarrow", true, "&crarr;", "<-'", "<-'", "↵"),
    "hookleftarrow" => make("\\hookleftarrow", true, "&crarr;",  "<-'", "<-'", "↵"),

    // Function names
    "arccos" => make("\\arccos", true, "arccos", "arccos", "arccos", "arccos"),
    "arcsin" => make("\\arcsin", true, "arcsin", "arcsin", "arcsin", "arcsin"),
    "arctan" => make("\\arctan", true, "arctan", "arctan", "arctan", "arctan"),
    "arg" => make("\\arg", true, "arg", "arg", "arg", "arg"),
    "cos" => make("\\cos", true, "cos", "cos", "cos", "cos"),
    "cosh" => make("\\cosh", true, "cosh", "cosh", "cosh", "cosh"),
    "cot" => make("\\cot", true, "cot", "cot", "cot", "cot"),
    "coth" => make("\\coth", true, "coth", "coth", "coth", "coth"),
    "csc" => make("\\csc", true, "csc", "csc", "csc", "csc"),
    //"deg" => make("\\deg", true, "&deg;", "deg", "deg", "deg"), // duplicate key
    "det" => make("\\det", true, "det", "det", "det", "det"),
    "dim" => make("\\dim", true, "dim", "dim", "dim", "dim"),
    "exp" => make("\\exp", true, "exp", "exp", "exp", "exp"),
    "gcd" => make("\\gcd", true, "gcd", "gcd", "gcd", "gcd"),
    "hom" => make("\\hom", true, "hom", "hom", "hom", "hom"),
    "inf" => make("\\inf", true, "inf", "inf", "inf", "inf"),
    "ker" => make("\\ker", true, "ker", "ker", "ker", "ker"),
    "lg" => make("\\lg", true, "lg", "lg", "lg", "lg"),
    "lim" => make("\\lim", true, "lim", "lim", "lim", "lim"),
    "liminf" => make("\\liminf", true, "liminf", "liminf", "liminf", "liminf"),
    "limsup" => make("\\limsup", true, "limsup", "limsup", "limsup", "limsup"),
    "ln" => make("\\ln", true, "ln", "ln", "ln", "ln"),
    "log" => make("\\log", true, "log", "log", "log", "log"),
    "max" => make("\\max", true, "max", "max", "max", "max"),
    "min" => make("\\min", true, "min", "min", "min", "min"),
    "Pr" => make("\\Pr", true, "Pr", "Pr", "Pr", "Pr"),
    "sec" => make("\\sec", true, "sec", "sec", "sec", "sec"),
    "sin" => make("\\sin", true, "sin", "sin", "sin", "sin"),
    "sinh" => make("\\sinh", true, "sinh", "sinh", "sinh", "sinh"),
    //"sup" => make("\\sup", true, "&sup;", "sup", "sup", "sup"), // duplicate key
    "tan" => make("\\tan", true, "tan", "tan", "tan", "tan"),
    "tanh" => make("\\tanh", true, "tanh", "tanh", "tanh", "tanh"),

    // Signs & Symbols
    "bull" => make("\\textbullet{}", false, "&bull;", "*", "*", "•"),
    "bullet" => make("\\textbullet{}", false, "&bull;", "*", "*", "•"),
    "star" => make("\\star", true, "*", "*", "*", "⋆"),
    "lowast" => make("\\ast", true, "&lowast;", "*", "*", "∗"),
    "ast" => make("\\ast", true, "&lowast;", "*", "*", "*"),
    "odot" => make("\\odot", true, "o", "[circled dot]", "[circled dot]", "ʘ"),
    "oplus" => make("\\oplus", true, "&oplus;", "[circled plus]", "[circled plus]", "⊕"),
    "otimes" => make("\\otimes", true, "&otimes;", "[circled times]", "[circled times]", "⊗"),
    "check" => make("\\checkmark", true, "&checkmark;", "[checkmark]", "[checkmark]", "✓"),
    "checkmark" => make("\\checkmark", true, "&check;", "[checkmark]", "[checkmark]", "✓"),

    // Miscellaneous (seldom used)
    "para" => make("\\P{}", false, "&para;", "[pilcrow]", "¶", "¶"),
    "ordf" => make("\\textordfeminine{}", false, "&ordf;", "_a_", "ª", "ª"),
    "ordm" => make("\\textordmasculine{}", false, "&ordm;", "_o_", "º", "º"),
    "cedil" => make("\\c{}", false, "&cedil;", "[cedilla]", "¸", "¸"),
    "oline" => make("\\overline{~}", true, "&oline;", "[overline]", "¯", "‾"),
    "uml" => make("\\textasciidieresis{}", false, "&uml;", "[diaeresis]", "¨", "¨"),
    "zwnj" => make("\\/{}", false, "&zwnj;", "", "", "‌"),
    "zwj" => make("", false, "&zwj;", "", "", "‍"),
    "lrm" => make("", false, "&lrm;", "", "", "‎"),
    "rlm" => make("", false, "&rlm;", "", "", "‏"),

    // Smilies
    "smiley" => make("\\ddot\\smile", true, "&#9786;", ":-)", ":-)", "☺"),
    "blacksmile" => make("\\ddot\\smile", true, "&#9787;", ":-)", ":-)", "☻"),
    "sad" => make("\\ddot\\frown", true, "&#9785;", ":-(", ":-(", "☹"),
    "frowny" => make("\\ddot\\frown", true, "&#9785;", ":-(", ":-(", "☹"),

    // Suits
    "clubs" => make("\\clubsuit", true, "&clubs;", "[clubs]", "[clubs]", "♣"),
    "clubsuit" => make("\\clubsuit", true, "&clubs;", "[clubs]", "[clubs]", "♣"),
    "spades" => make("\\spadesuit", true, "&spades;", "[spades]", "[spades]", "♠"),
    "spadesuit" => make("\\spadesuit", true, "&spades;", "[spades]", "[spades]", "♠"),
    "hearts" => make("\\heartsuit", true, "&hearts;", "[hearts]", "[hearts]", "♥"),
    "heartsuit" => make("\\heartsuit", true, "&heartsuit;", "[hearts]", "[hearts]", "♥"),
    "diams" => make("\\diamondsuit", true, "&diams;", "[diamonds]", "[diamonds]", "◆"),
    "diamondsuit" => make("\\diamondsuit", true, "&diams;", "[diamonds]", "[diamonds]", "◆"),
    "diamond" => make("\\diamondsuit", true, "&diamond;", "[diamond]", "[diamond]", "◆"),
    "Diamond" => make("\\diamondsuit", true, "&diamond;", "[diamond]", "[diamond]", "◆"),
    "loz" => make("\\lozenge", true, "&loz;", "[lozenge]", "[lozenge]", "⧫"),

    // TODO needs build script to include at compile time
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
};
