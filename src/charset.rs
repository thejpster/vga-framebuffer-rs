/// This MS-DOS CodePage 850. It offers a compromise between the box
/// characters of CodePage 437 and the accents of ISO 8859-1 / Latin-1.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Char {
    Null,                                   // Char 000 ((' ', 1)) = REPLACEMENT CHARACTER
    SOH,                                    // Char 001 (('☺', 1)) = WHITE SMILING FACE
    STX,                                    // Char 002 (('☻', 1)) = BLACK SMILING FACE
    ETX,                                    // Char 003 (('♥', 1)) = BLACK HEART SUIT
    EOT,                                    // Char 004 (('♦', 1)) = BLACK DIAMOND SUIT
    ENQ,                                    // Char 005 (('♣', 1)) = BLACK CLUB SUIT
    ACK,                                    // Char 006 (('♠', 1)) = BLACK SPADE SUIT
    BEL,                                    // Char 007 (('•', 1)) = BULLET
    BS,                                     // Char 008 (('◘', 1)) = INVERSE BULLET
    HT,                                     // Char 009 (('○', 1)) = WHITE CIRCLE
    LF,                                     // Char 010 (('◙', 1)) = INVERSE WHITE CIRCLE
    VT,                                     // Char 011 (('♂', 1)) = MALE SIGN
    FF,                                     // Char 012 (('♀', 1)) = FEMALE SIGN
    CR,                                     // Char 013 (('♪', 1)) = EIGHTH NOTE
    SO,                                     // Char 014 (('♫', 1)) = BEAMED EIGHTH NOTES
    SI,                                     // Char 015 (('☼', 1)) = WHITE SUN WITH RAYS
    DLE,                                    // Char 016 (('►', 1)) = BLACK RIGHT-POINTING ARROW
    DC1,                                    // Char 017 (('◄', 1)) = BLACK LEFT-POINTING ARROW
    DC2,                                    // Char 018 (('↕', 1)) = UP DOWN ARROW
    DC3,                                    // Char 019 (('‼', 1)) = DOUBLE EXCLAMATION MARK
    DC4,                                    // Char 020 (('¶', 1)) = PILCROW SIGN
    NAK,                                    // Char 021 (('§', 1)) = SECTION SIGN
    SYN,                                    // Char 022 (('▬', 1)) = BLACK RECTANGLE
    ETB,                                    // Char 023 (('↨', 1)) = UP DOWN ARROW WITH BASE
    CAN,                                    // Char 024 (('↑', 1)) = UPWARDS ARROW
    EM,                                     // Char 025 (('↓', 1)) = DOWNWARDS ARROW
    SUB,                                    // Char 026 (('→', 1)) = RIGHTWARDS ARROW
    Escape,                                 // Char 027 (('←', 1)) = LEFTWARDS ARROW
    FS,                                     // Char 028 (('∟', 1)) = RIGHT ANDLE
    GS,                                     // Char 029 (('↔', 1)) = LEFT RIGHT ARROW
    RS,                                     // Char 030 (('▲', 1)) = BLACK UP POINTING ARROW
    US,                                     // Char 031 (('▼', 1)) = BLACK DOWN POINTING ARROW
    Space,                                  // Char 032 ((' ', 1)) = SPACE
    ExclamationMark,                        // Char 033 (('!', 1)) = EXCLAMATION MARK
    QuotationMark,                          // Char 034 (('"', 1)) = QUOTATION MARK
    NumberSign,                             // Char 035 (('#', 1)) = NUMBER SIGN
    DollarSign,                             // Char 036 (('$', 1)) = DOLLAR SIGN
    PercentSign,                            // Char 037 (('%', 1)) = PERCENT SIGN
    Ampersand,                              // Char 038 (('&', 1)) = AMPERSAND
    Apostrophe,                             // Char 039 (("'", 1)) = APOSTROPHE
    LeftParenthesis,                        // Char 040 (('(', 1)) = LEFT PARENTHESIS
    RightParenthesis,                       // Char 041 ((')', 1)) = RIGHT PARENTHESIS
    Asterisk,                               // Char 042 (('*', 1)) = ASTERISK
    PlusSign,                               // Char 043 (('+', 1)) = PLUS SIGN
    Comma,                                  // Char 044 ((',', 1)) = COMMA
    HyphenMinus,                            // Char 045 (('-', 1)) = HYPHEN-MINUS
    FullStop,                               // Char 046 (('.', 1)) = FULL STOP
    Solidus,                                // Char 047 (('/', 1)) = SOLIDUS
    DigitZero,                              // Char 048 (('0', 1)) = DIGIT ZERO
    DigitOne,                               // Char 049 (('1', 1)) = DIGIT ONE
    DigitTwo,                               // Char 050 (('2', 1)) = DIGIT TWO
    DigitThree,                             // Char 051 (('3', 1)) = DIGIT THREE
    DigitFour,                              // Char 052 (('4', 1)) = DIGIT FOUR
    DigitFive,                              // Char 053 (('5', 1)) = DIGIT FIVE
    DigitSix,                               // Char 054 (('6', 1)) = DIGIT SIX
    DigitSeven,                             // Char 055 (('7', 1)) = DIGIT SEVEN
    DigitEight,                             // Char 056 (('8', 1)) = DIGIT EIGHT
    DigitNine,                              // Char 057 (('9', 1)) = DIGIT NINE
    Colon,                                  // Char 058 ((':', 1)) = COLON
    Semicolon,                              // Char 059 ((';', 1)) = SEMICOLON
    LessThanSign,                           // Char 060 (('<', 1)) = LESS-THAN SIGN
    EqualsSign,                             // Char 061 (('=', 1)) = EQUALS SIGN
    GreaterThanSign,                        // Char 062 (('>', 1)) = GREATER-THAN SIGN
    QuestionMark,                           // Char 063 (('?', 1)) = QUESTION MARK
    CommercialAt,                           // Char 064 (('@', 1)) = COMMERCIAL AT
    LatinCapitalLetterA,                    // Char 065 (('A', 1)) = LATIN CAPITAL LETTER A
    LatinCapitalLetterB,                    // Char 066 (('B', 1)) = LATIN CAPITAL LETTER B
    LatinCapitalLetterC,                    // Char 067 (('C', 1)) = LATIN CAPITAL LETTER C
    LatinCapitalLetterD,                    // Char 068 (('D', 1)) = LATIN CAPITAL LETTER D
    LatinCapitalLetterE,                    // Char 069 (('E', 1)) = LATIN CAPITAL LETTER E
    LatinCapitalLetterF,                    // Char 070 (('F', 1)) = LATIN CAPITAL LETTER F
    LatinCapitalLetterG,                    // Char 071 (('G', 1)) = LATIN CAPITAL LETTER G
    LatinCapitalLetterH,                    // Char 072 (('H', 1)) = LATIN CAPITAL LETTER H
    LatinCapitalLetterI,                    // Char 073 (('I', 1)) = LATIN CAPITAL LETTER I
    LatinCapitalLetterJ,                    // Char 074 (('J', 1)) = LATIN CAPITAL LETTER J
    LatinCapitalLetterK,                    // Char 075 (('K', 1)) = LATIN CAPITAL LETTER K
    LatinCapitalLetterL,                    // Char 076 (('L', 1)) = LATIN CAPITAL LETTER L
    LatinCapitalLetterM,                    // Char 077 (('M', 1)) = LATIN CAPITAL LETTER M
    LatinCapitalLetterN,                    // Char 078 (('N', 1)) = LATIN CAPITAL LETTER N
    LatinCapitalLetterO,                    // Char 079 (('O', 1)) = LATIN CAPITAL LETTER O
    LatinCapitalLetterP,                    // Char 080 (('P', 1)) = LATIN CAPITAL LETTER P
    LatinCapitalLetterQ,                    // Char 081 (('Q', 1)) = LATIN CAPITAL LETTER Q
    LatinCapitalLetterR,                    // Char 082 (('R', 1)) = LATIN CAPITAL LETTER R
    LatinCapitalLetterS,                    // Char 083 (('S', 1)) = LATIN CAPITAL LETTER S
    LatinCapitalLetterT,                    // Char 084 (('T', 1)) = LATIN CAPITAL LETTER T
    LatinCapitalLetterU,                    // Char 085 (('U', 1)) = LATIN CAPITAL LETTER U
    LatinCapitalLetterV,                    // Char 086 (('V', 1)) = LATIN CAPITAL LETTER V
    LatinCapitalLetterW,                    // Char 087 (('W', 1)) = LATIN CAPITAL LETTER W
    LatinCapitalLetterX,                    // Char 088 (('X', 1)) = LATIN CAPITAL LETTER X
    LatinCapitalLetterY,                    // Char 089 (('Y', 1)) = LATIN CAPITAL LETTER Y
    LatinCapitalLetterZ,                    // Char 090 (('Z', 1)) = LATIN CAPITAL LETTER Z
    LeftSquareBracket,                      // Char 091 (('[', 1)) = LEFT SQUARE BRACKET
    ReverseSolidus,                         // Char 092 (('\\', 1)) = REVERSE SOLIDUS
    RightSquareBracket,                     // Char 093 ((']', 1)) = RIGHT SQUARE BRACKET
    CircumflexAccent,                       // Char 094 (('^', 1)) = CIRCUMFLEX ACCENT
    LowLine,                                // Char 095 (('_', 1)) = LOW LINE
    GraveAccent,                            // Char 096 (('`', 1)) = GRAVE ACCENT
    LatinSmallLetterA,                      // Char 097 (('a', 1)) = LATIN SMALL LETTER A
    LatinSmallLetterB,                      // Char 098 (('b', 1)) = LATIN SMALL LETTER B
    LatinSmallLetterC,                      // Char 099 (('c', 1)) = LATIN SMALL LETTER C
    LatinSmallLetterD,                      // Char 100 (('d', 1)) = LATIN SMALL LETTER D
    LatinSmallLetterE,                      // Char 101 (('e', 1)) = LATIN SMALL LETTER E
    LatinSmallLetterF,                      // Char 102 (('f', 1)) = LATIN SMALL LETTER F
    LatinSmallLetterG,                      // Char 103 (('g', 1)) = LATIN SMALL LETTER G
    LatinSmallLetterH,                      // Char 104 (('h', 1)) = LATIN SMALL LETTER H
    LatinSmallLetterI,                      // Char 105 (('i', 1)) = LATIN SMALL LETTER I
    LatinSmallLetterJ,                      // Char 106 (('j', 1)) = LATIN SMALL LETTER J
    LatinSmallLetterK,                      // Char 107 (('k', 1)) = LATIN SMALL LETTER K
    LatinSmallLetterL,                      // Char 108 (('l', 1)) = LATIN SMALL LETTER L
    LatinSmallLetterM,                      // Char 109 (('m', 1)) = LATIN SMALL LETTER M
    LatinSmallLetterN,                      // Char 110 (('n', 1)) = LATIN SMALL LETTER N
    LatinSmallLetterO,                      // Char 111 (('o', 1)) = LATIN SMALL LETTER O
    LatinSmallLetterP,                      // Char 112 (('p', 1)) = LATIN SMALL LETTER P
    LatinSmallLetterQ,                      // Char 113 (('q', 1)) = LATIN SMALL LETTER Q
    LatinSmallLetterR,                      // Char 114 (('r', 1)) = LATIN SMALL LETTER R
    LatinSmallLetterS,                      // Char 115 (('s', 1)) = LATIN SMALL LETTER S
    LatinSmallLetterT,                      // Char 116 (('t', 1)) = LATIN SMALL LETTER T
    LatinSmallLetterU,                      // Char 117 (('u', 1)) = LATIN SMALL LETTER U
    LatinSmallLetterV,                      // Char 118 (('v', 1)) = LATIN SMALL LETTER V
    LatinSmallLetterW,                      // Char 119 (('w', 1)) = LATIN SMALL LETTER W
    LatinSmallLetterX,                      // Char 120 (('x', 1)) = LATIN SMALL LETTER X
    LatinSmallLetterY,                      // Char 121 (('y', 1)) = LATIN SMALL LETTER Y
    LatinSmallLetterZ,                      // Char 122 (('z', 1)) = LATIN SMALL LETTER Z
    LeftCurlyBracket,                       // Char 123 (('{', 1)) = LEFT CURLY BRACKET
    VerticalLine,                           // Char 124 (('|', 1)) = VERTICAL LINE
    RightCurlyBracket,                      // Char 125 (('}', 1)) = RIGHT CURLY BRACKET
    Tilde,                                  // Char 126 (('~', 1)) = TILDE
    Delete,                                 // Char 127 (('\x7f', 1)) = DELETE
    LatinCapitalLetterCWithCedilla, // Char 128 (('Ç', 1)) = LATIN CAPITAL LETTER C WITH CEDILLA
    LatinSmallLetterUWithDiaeresis, // Char 129 (('ü', 1)) = LATIN SMALL LETTER U WITH DIAERESIS
    LatinSmallLetterEWithAcute,     // Char 130 (('é', 1)) = LATIN SMALL LETTER E WITH ACUTE
    LatinSmallLetterAWithCircumflex, // Char 131 (('â', 1)) = LATIN SMALL LETTER A WITH CIRCUMFLEX
    LatinSmallLetterAWithDiaeresis, // Char 132 (('ä', 1)) = LATIN SMALL LETTER A WITH DIAERESIS
    LatinSmallLetterAWithGrave,     // Char 133 (('à', 1)) = LATIN SMALL LETTER A WITH GRAVE
    LatinSmallLetterAWithRingAbove, // Char 134 (('å', 1)) = LATIN SMALL LETTER A WITH RING ABOVE
    LatinSmallLetterCWithCedilla,   // Char 135 (('ç', 1)) = LATIN SMALL LETTER C WITH CEDILLA
    LatinSmallLetterEWithCircumflex, // Char 136 (('ê', 1)) = LATIN SMALL LETTER E WITH CIRCUMFLEX
    LatinSmallLetterEWithDiaeresis, // Char 137 (('ë', 1)) = LATIN SMALL LETTER E WITH DIAERESIS
    LatinSmallLetterEWithGrave,     // Char 138 (('è', 1)) = LATIN SMALL LETTER E WITH GRAVE
    LatinSmallLetterIWithDiaeresis, // Char 139 (('ï', 1)) = LATIN SMALL LETTER I WITH DIAERESIS
    LatinSmallLetterIWithCircumflex, // Char 140 (('î', 1)) = LATIN SMALL LETTER I WITH CIRCUMFLEX
    LatinSmallLetterIWithGrave,     // Char 141 (('ì', 1)) = LATIN SMALL LETTER I WITH GRAVE
    LatinCapitalLetterAWithDiaeresis, // Char 142 (('Ä', 1)) = LATIN CAPITAL LETTER A WITH DIAERESIS
    LatinCapitalLetterAWithRingAbove, // Char 143 (('Å', 1)) = LATIN CAPITAL LETTER A WITH RING ABOVE
    LatinCapitalLetterEWithAcute,     // Char 144 (('É', 1)) = LATIN CAPITAL LETTER E WITH ACUTE
    LatinSmallLetterAe,               // Char 145 (('æ', 1)) = LATIN SMALL LETTER AE
    LatinCapitalLetterAe,             // Char 146 (('Æ', 1)) = LATIN CAPITAL LETTER AE
    LatinSmallLetterOWithCircumflex, // Char 147 (('ô', 1)) = LATIN SMALL LETTER O WITH CIRCUMFLEX
    LatinSmallLetterOWithDiaeresis,  // Char 148 (('ö', 1)) = LATIN SMALL LETTER O WITH DIAERESIS
    LatinSmallLetterOWithGrave,      // Char 149 (('ò', 1)) = LATIN SMALL LETTER O WITH GRAVE
    LatinSmallLetterUWithCircumflex, // Char 150 (('û', 1)) = LATIN SMALL LETTER U WITH CIRCUMFLEX
    LatinSmallLetterUWithGrave,      // Char 151 (('ù', 1)) = LATIN SMALL LETTER U WITH GRAVE
    LatinSmallLetterYWithDiaeresis,  // Char 152 (('ÿ', 1)) = LATIN SMALL LETTER Y WITH DIAERESIS
    LatinCapitalLetterOWithDiaeresis, // Char 153 (('Ö', 1)) = LATIN CAPITAL LETTER O WITH DIAERESIS
    LatinCapitalLetterUWithDiaeresis, // Char 154 (('Ü', 1)) = LATIN CAPITAL LETTER U WITH DIAERESIS
    LatinSmallLetterOWithStroke,      // Char 155 (('ø', 1)) = LATIN SMALL LETTER O WITH STROKE
    PoundSign,                        // Char 156 (('£', 1)) = POUND SIGN
    LatinCapitalLetterOWithStroke,    // Char 157 (('Ø', 1)) = LATIN CAPITAL LETTER O WITH STROKE
    MultiplicationSign,               // Char 158 (('×', 1)) = MULTIPLICATION SIGN
    LatinSmallLetterFWithHook,        // Char 159 (('ƒ', 1)) = LATIN SMALL LETTER F WITH HOOK
    LatinSmallLetterAWithAcute,       // Char 160 (('á', 1)) = LATIN SMALL LETTER A WITH ACUTE
    LatinSmallLetterIWithAcute,       // Char 161 (('í', 1)) = LATIN SMALL LETTER I WITH ACUTE
    LatinSmallLetterOWithAcute,       // Char 162 (('ó', 1)) = LATIN SMALL LETTER O WITH ACUTE
    LatinSmallLetterUWithAcute,       // Char 163 (('ú', 1)) = LATIN SMALL LETTER U WITH ACUTE
    LatinSmallLetterNWithTilde,       // Char 164 (('ñ', 1)) = LATIN SMALL LETTER N WITH TILDE
    LatinCapitalLetterNWithTilde,     // Char 165 (('Ñ', 1)) = LATIN CAPITAL LETTER N WITH TILDE
    FeminineOrdinalIndicator,         // Char 166 (('ª', 1)) = FEMININE ORDINAL INDICATOR
    MasculineOrdinalIndicator,        // Char 167 (('º', 1)) = MASCULINE ORDINAL INDICATOR
    InvertedQuestionMark,             // Char 168 (('¿', 1)) = INVERTED QUESTION MARK
    RegisteredSign,                   // Char 169 (('®', 1)) = REGISTERED SIGN
    NotSign,                          // Char 170 (('¬', 1)) = NOT SIGN
    VulgarFractionOneHalf,            // Char 171 (('½', 1)) = VULGAR FRACTION ONE HALF
    VulgarFractionOneQuarter,         // Char 172 (('¼', 1)) = VULGAR FRACTION ONE QUARTER
    InvertedExclamationMark,          // Char 173 (('¡', 1)) = INVERTED EXCLAMATION MARK
    LeftPointingDoubleAngleQuotationMark, // Char 174 (('«', 1)) = LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    RightPointingDoubleAngleQuotationMark, // Char 175 (('»', 1)) = RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    LightShade,                            // Char 176 (('░', 1)) = LIGHT SHADE
    MediumShade,                           // Char 177 (('▒', 1)) = MEDIUM SHADE
    DarkShade,                             // Char 178 (('▓', 1)) = DARK SHADE
    BoxDrawingsLightVertical,              // Char 179 (('│', 1)) = BOX DRAWINGS LIGHT VERTICAL
    BoxDrawingsLightVerticalAndLeft, // Char 180 (('┤', 1)) = BOX DRAWINGS LIGHT VERTICAL AND LEFT
    LatinCapitalLetterAWithAcute,    // Char 181 (('Á', 1)) = LATIN CAPITAL LETTER A WITH ACUTE
    LatinCapitalLetterAWithCircumflex, // Char 182 (('Â', 1)) = LATIN CAPITAL LETTER A WITH CIRCUMFLEX
    LatinCapitalLetterAWithGrave,      // Char 183 (('À', 1)) = LATIN CAPITAL LETTER A WITH GRAVE
    CopyrightSign,                     // Char 184 (('©', 1)) = COPYRIGHT SIGN
    BoxDrawingsDoubleVerticalAndLeft, // Char 185 (('╣', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    BoxDrawingsDoubleVertical,        // Char 186 (('║', 1)) = BOX DRAWINGS DOUBLE VERTICAL
    BoxDrawingsDoubleDownAndLeft,     // Char 187 (('╗', 1)) = BOX DRAWINGS DOUBLE DOWN AND LEFT
    BoxDrawingsDoubleUpAndLeft,       // Char 188 (('╝', 1)) = BOX DRAWINGS DOUBLE UP AND LEFT
    CentSign,                         // Char 189 (('¢', 1)) = CENT SIGN
    YenSign,                          // Char 190 (('¥', 1)) = YEN SIGN
    BoxDrawingsLightDownAndLeft,      // Char 191 (('┐', 1)) = BOX DRAWINGS LIGHT DOWN AND LEFT
    BoxDrawingsLightUpAndRight,       // Char 192 (('└', 1)) = BOX DRAWINGS LIGHT UP AND RIGHT
    BoxDrawingsLightUpAndHorizontal, // Char 193 (('┴', 1)) = BOX DRAWINGS LIGHT UP AND HORIZONTAL
    BoxDrawingsLightDownAndHorizontal, // Char 194 (('┬', 1)) = BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    BoxDrawingsLightVerticalAndRight, // Char 195 (('├', 1)) = BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    BoxDrawingsLightHorizontal,       // Char 196 (('─', 1)) = BOX DRAWINGS LIGHT HORIZONTAL
    BoxDrawingsLightVerticalAndHorizontal, // Char 197 (('┼', 1)) = BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    LatinSmallLetterAWithTilde, // Char 198 (('ã', 1)) = LATIN SMALL LETTER A WITH TILDE
    LatinCapitalLetterAWithTilde, // Char 199 (('Ã', 1)) = LATIN CAPITAL LETTER A WITH TILDE
    BoxDrawingsDoubleUpAndRight, // Char 200 (('╚', 1)) = BOX DRAWINGS DOUBLE UP AND RIGHT
    BoxDrawingsDoubleDownAndRight, // Char 201 (('╔', 1)) = BOX DRAWINGS DOUBLE DOWN AND RIGHT
    BoxDrawingsDoubleUpAndHorizontal, // Char 202 (('╩', 1)) = BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    BoxDrawingsDoubleDownAndHorizontal, // Char 203 (('╦', 1)) = BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    BoxDrawingsDoubleVerticalAndRight, // Char 204 (('╠', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    BoxDrawingsDoubleHorizontal,       // Char 205 (('═', 1)) = BOX DRAWINGS DOUBLE HORIZONTAL
    BoxDrawingsDoubleVerticalAndHorizontal, // Char 206 (('╬', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    CurrencySign,                           // Char 207 (('¤', 1)) = CURRENCY SIGN
    LatinSmallLetterEth,                    // Char 208 (('ð', 1)) = LATIN SMALL LETTER ETH
    LatinCapitalLetterEth,                  // Char 209 (('Ð', 1)) = LATIN CAPITAL LETTER ETH
    LatinCapitalLetterEWithCircumflex, // Char 210 (('Ê', 1)) = LATIN CAPITAL LETTER E WITH CIRCUMFLEX
    LatinCapitalLetterEWithDiaeresis, // Char 211 (('Ë', 1)) = LATIN CAPITAL LETTER E WITH DIAERESIS
    LatinCapitalLetterEWithGrave,     // Char 212 (('È', 1)) = LATIN CAPITAL LETTER E WITH GRAVE
    LatinSmallLetterDotlessI,         // Char 213 (('ı', 1)) = LATIN SMALL LETTER DOTLESS I
    LatinCapitalLetterIWithAcute,     // Char 214 (('Í', 1)) = LATIN CAPITAL LETTER I WITH ACUTE
    LatinCapitalLetterIWithCircumflex, // Char 215 (('Î', 1)) = LATIN CAPITAL LETTER I WITH CIRCUMFLEX
    LatinCapitalLetterIWithDiaeresis, // Char 216 (('Ï', 1)) = LATIN CAPITAL LETTER I WITH DIAERESIS
    BoxDrawingsLightUpAndLeft,        // Char 217 (('┘', 1)) = BOX DRAWINGS LIGHT UP AND LEFT
    BoxDrawingsLightDownAndRight,     // Char 218 (('┌', 1)) = BOX DRAWINGS LIGHT DOWN AND RIGHT
    FullBlock,                        // Char 219 (('█', 1)) = FULL BLOCK
    LowerHalfBlock,                   // Char 220 (('▄', 1)) = LOWER HALF BLOCK
    BrokenBar,                        // Char 221 (('¦', 1)) = BROKEN BAR
    LatinCapitalLetterIWithGrave,     // Char 222 (('Ì', 1)) = LATIN CAPITAL LETTER I WITH GRAVE
    UpperHalfBlock,                   // Char 223 (('▀', 1)) = UPPER HALF BLOCK
    LatinCapitalLetterOWithAcute,     // Char 224 (('Ó', 1)) = LATIN CAPITAL LETTER O WITH ACUTE
    LatinSmallLetterSharpS,           // Char 225 (('ß', 1)) = LATIN SMALL LETTER SHARP S
    LatinCapitalLetterOWithCircumflex, // Char 226 (('Ô', 1)) = LATIN CAPITAL LETTER O WITH CIRCUMFLEX
    LatinCapitalLetterOWithGrave,      // Char 227 (('Ò', 1)) = LATIN CAPITAL LETTER O WITH GRAVE
    LatinSmallLetterOWithTilde,        // Char 228 (('õ', 1)) = LATIN SMALL LETTER O WITH TILDE
    LatinCapitalLetterOWithTilde,      // Char 229 (('Õ', 1)) = LATIN CAPITAL LETTER O WITH TILDE
    MicroSign,                         // Char 230 (('µ', 1)) = MICRO SIGN
    LatinSmallLetterThorn,             // Char 231 (('þ', 1)) = LATIN SMALL LETTER THORN
    LatinCapitalLetterThorn,           // Char 232 (('Þ', 1)) = LATIN CAPITAL LETTER THORN
    LatinCapitalLetterUWithAcute,      // Char 233 (('Ú', 1)) = LATIN CAPITAL LETTER U WITH ACUTE
    LatinCapitalLetterUWithCircumflex, // Char 234 (('Û', 1)) = LATIN CAPITAL LETTER U WITH CIRCUMFLEX
    LatinCapitalLetterUWithGrave,      // Char 235 (('Ù', 1)) = LATIN CAPITAL LETTER U WITH GRAVE
    LatinSmallLetterYWithAcute,        // Char 236 (('ý', 1)) = LATIN SMALL LETTER Y WITH ACUTE
    LatinCapitalLetterYWithAcute,      // Char 237 (('Ý', 1)) = LATIN CAPITAL LETTER Y WITH ACUTE
    Macron,                            // Char 238 (('¯', 1)) = MACRON
    AcuteAccent,                       // Char 239 (('´', 1)) = ACUTE ACCENT
    SoftHyphen,                        // Char 240 (('­­­­­­-', 1)) = SOFT HYPHEN
    PlusMinusSign,                     // Char 241 (('±', 1)) = PLUS-MINUS SIGN
    DoubleLowLine,                     // Char 242 (('‗', 1)) = DOUBLE LOW LINE
    VulgarFractionThreeQuarters,       // Char 243 (('¾', 1)) = VULGAR FRACTION THREE QUARTERS
    PilcrowSign,                       // Char 244 (('¶', 1)) = PILCROW SIGN
    SectionSign,                       // Char 245 (('§', 1)) = SECTION SIGN
    DivisionSign,                      // Char 246 (('÷', 1)) = DIVISION SIGN
    Cedilla,                           // Char 247 (('¸', 1)) = CEDILLA
    DegreeSign,                        // Char 248 (('°', 1)) = DEGREE SIGN
    Diaeresis,                         // Char 249 (('¨', 1)) = DIAERESIS
    MiddleDot,                         // Char 250 (('·', 1)) = MIDDLE DOT
    SuperscriptOne,                    // Char 251 (('¹', 1)) = SUPERSCRIPT ONE
    SuperscriptThree,                  // Char 252 (('³', 1)) = SUPERSCRIPT THREE
    SuperscriptTwo,                    // Char 253 (('²', 1)) = SUPERSCRIPT TWO
    BlackSquare,                       // Char 254 (('■', 1)) = BLACK SQUARE
    NoBreakSpace,                      // Char 255 (('\xa0', 1)) = NO-BREAK SPACE
}

impl ::core::default::Default for Char {
    fn default() -> Char {
        Char::Space
    }
}

impl Char {
    pub fn map_char(ch: char) -> Char {
        match ch {
            // Add more CP850 mappings here - hearts, clubs, diamonds, etc
            '\u{0000}' => Char::Null,   // Char 0 (('\x00', 1)) = UNKNOWN
            '\u{0001}' => Char::SOH,    // Char 1 (('\x01', 1)) = UNKNOWN
            '\u{0002}' => Char::STX,    // Char 2 (('\x02', 1)) = UNKNOWN
            '\u{0003}' => Char::ETX,    // Char 3 (('\x03', 1)) = UNKNOWN
            '\u{0004}' => Char::EOT,    // Char 4 (('\x04', 1)) = UNKNOWN
            '\u{0005}' => Char::ENQ,    // Char 5 (('\x05', 1)) = UNKNOWN
            '\u{0006}' => Char::ACK,    // Char 6 (('\x06', 1)) = UNKNOWN
            '\u{0007}' => Char::BEL,    // Char 7 (('\x07', 1)) = UNKNOWN
            '\u{0008}' => Char::BS,     // Char 8 (('\x08', 1)) = UNKNOWN
            '\t' => Char::HT,           // Char 9 (('\t', 1)) = UNKNOWN
            '\n' => Char::LF,           // Char 10 (('\n', 1)) = UNKNOWN
            '\u{000b}' => Char::VT,     // Char 11 (('\x0b', 1)) = UNKNOWN
            '\u{000c}' => Char::FF,     // Char 12 (('\x0c', 1)) = UNKNOWN
            '\r' => Char::CR,           // Char 13 (('\r', 1)) = UNKNOWN
            '\u{000e}' => Char::SO,     // Char 14 (('\x0e', 1)) = UNKNOWN
            '\u{000f}' => Char::SI,     // Char 15 (('\x0f', 1)) = UNKNOWN
            '\u{0010}' => Char::DLE,    // Char 16 (('\x10', 1)) = UNKNOWN
            '\u{0011}' => Char::DC1,    // Char 17 (('\x11', 1)) = UNKNOWN
            '\u{0012}' => Char::DC2,    // Char 18 (('\x12', 1)) = UNKNOWN
            '\u{0013}' => Char::DC3,    // Char 19 (('\x13', 1)) = UNKNOWN
            '\u{0014}' => Char::DC4,    // Char 20 (('\x14', 1)) = UNKNOWN
            '\u{0015}' => Char::NAK,    // Char 21 (('\x15', 1)) = UNKNOWN
            '\u{0016}' => Char::SYN,    // Char 22 (('\x16', 1)) = UNKNOWN
            '\u{0017}' => Char::ETB,    // Char 23 (('\x17', 1)) = UNKNOWN
            '\u{0018}' => Char::CAN,    // Char 24 (('\x18', 1)) = UNKNOWN
            '\u{0019}' => Char::EM,     // Char 25 (('\x19', 1)) = UNKNOWN
            '\u{001a}' => Char::SUB,    // Char 26 (('\x1a', 1)) = UNKNOWN
            '\u{001b}' => Char::Escape, // Char 27 (('\x1b', 1)) = UNKNOWN
            '\u{001c}' => Char::FS,     // Char 28 (('\x1c', 1)) = UNKNOWN
            '\u{001d}' => Char::GS,     // Char 29 (('\x1d', 1)) = UNKNOWN
            '\u{001e}' => Char::RS,     // Char 30 (('\x1e', 1)) = UNKNOWN
            '\u{001f}' => Char::US,     // Char 31 (('\x1f', 1)) = UNKNOWN
            ' ' => Char::Space,         // Char 32 ((' ', 1)) = SPACE
            '!' => Char::ExclamationMark, // Char 33 (('!', 1)) = EXCLAMATION MARK
            '"' => Char::QuotationMark, // Char 34 (('"', 1)) = QUOTATION MARK
            '#' => Char::NumberSign,    // Char 35 (('#', 1)) = NUMBER SIGN
            '$' => Char::DollarSign,    // Char 36 (('$', 1)) = DOLLAR SIGN
            '%' => Char::PercentSign,   // Char 37 (('%', 1)) = PERCENT SIGN
            '&' => Char::Ampersand,     // Char 38 (('&', 1)) = AMPERSAND
            '\'' => Char::Apostrophe,   // Char 39 (("'", 1)) = APOSTROPHE
            '(' => Char::LeftParenthesis, // Char 40 (('(', 1)) = LEFT PARENTHESIS
            ')' => Char::RightParenthesis, // Char 41 ((')', 1)) = RIGHT PARENTHESIS
            '*' => Char::Asterisk,      // Char 42 (('*', 1)) = ASTERISK
            '+' => Char::PlusSign,      // Char 43 (('+', 1)) = PLUS SIGN
            ',' => Char::Comma,         // Char 44 ((',', 1)) = COMMA
            '-' => Char::HyphenMinus,   // Char 45 (('-', 1)) = HYPHEN-MINUS
            '.' => Char::FullStop,      // Char 46 (('.', 1)) = FULL STOP
            '/' => Char::Solidus,       // Char 47 (('/', 1)) = SOLIDUS
            '0' => Char::DigitZero,     // Char 48 (('0', 1)) = DIGIT ZERO
            '1' => Char::DigitOne,      // Char 49 (('1', 1)) = DIGIT ONE
            '2' => Char::DigitTwo,      // Char 50 (('2', 1)) = DIGIT TWO
            '3' => Char::DigitThree,    // Char 51 (('3', 1)) = DIGIT THREE
            '4' => Char::DigitFour,     // Char 52 (('4', 1)) = DIGIT FOUR
            '5' => Char::DigitFive,     // Char 53 (('5', 1)) = DIGIT FIVE
            '6' => Char::DigitSix,      // Char 54 (('6', 1)) = DIGIT SIX
            '7' => Char::DigitSeven,    // Char 55 (('7', 1)) = DIGIT SEVEN
            '8' => Char::DigitEight,    // Char 56 (('8', 1)) = DIGIT EIGHT
            '9' => Char::DigitNine,     // Char 57 (('9', 1)) = DIGIT NINE
            ':' => Char::Colon,         // Char 58 ((':', 1)) = COLON
            ';' => Char::Semicolon,     // Char 59 ((';', 1)) = SEMICOLON
            '<' => Char::LessThanSign,  // Char 60 (('<', 1)) = LESS-THAN SIGN
            '=' => Char::EqualsSign,    // Char 61 (('=', 1)) = EQUALS SIGN
            '>' => Char::GreaterThanSign, // Char 62 (('>', 1)) = GREATER-THAN SIGN
            '?' => Char::QuestionMark,  // Char 63 (('?', 1)) = QUESTION MARK
            '@' => Char::CommercialAt,  // Char 64 (('@', 1)) = COMMERCIAL AT
            'A' => Char::LatinCapitalLetterA, // Char 65 (('A', 1)) = LATIN CAPITAL LETTER A
            'B' => Char::LatinCapitalLetterB, // Char 66 (('B', 1)) = LATIN CAPITAL LETTER B
            'C' => Char::LatinCapitalLetterC, // Char 67 (('C', 1)) = LATIN CAPITAL LETTER C
            'D' => Char::LatinCapitalLetterD, // Char 68 (('D', 1)) = LATIN CAPITAL LETTER D
            'E' => Char::LatinCapitalLetterE, // Char 69 (('E', 1)) = LATIN CAPITAL LETTER E
            'F' => Char::LatinCapitalLetterF, // Char 70 (('F', 1)) = LATIN CAPITAL LETTER F
            'G' => Char::LatinCapitalLetterG, // Char 71 (('G', 1)) = LATIN CAPITAL LETTER G
            'H' => Char::LatinCapitalLetterH, // Char 72 (('H', 1)) = LATIN CAPITAL LETTER H
            'I' => Char::LatinCapitalLetterI, // Char 73 (('I', 1)) = LATIN CAPITAL LETTER I
            'J' => Char::LatinCapitalLetterJ, // Char 74 (('J', 1)) = LATIN CAPITAL LETTER J
            'K' => Char::LatinCapitalLetterK, // Char 75 (('K', 1)) = LATIN CAPITAL LETTER K
            'L' => Char::LatinCapitalLetterL, // Char 76 (('L', 1)) = LATIN CAPITAL LETTER L
            'M' => Char::LatinCapitalLetterM, // Char 77 (('M', 1)) = LATIN CAPITAL LETTER M
            'N' => Char::LatinCapitalLetterN, // Char 78 (('N', 1)) = LATIN CAPITAL LETTER N
            'O' => Char::LatinCapitalLetterO, // Char 79 (('O', 1)) = LATIN CAPITAL LETTER O
            'P' => Char::LatinCapitalLetterP, // Char 80 (('P', 1)) = LATIN CAPITAL LETTER P
            'Q' => Char::LatinCapitalLetterQ, // Char 81 (('Q', 1)) = LATIN CAPITAL LETTER Q
            'R' => Char::LatinCapitalLetterR, // Char 82 (('R', 1)) = LATIN CAPITAL LETTER R
            'S' => Char::LatinCapitalLetterS, // Char 83 (('S', 1)) = LATIN CAPITAL LETTER S
            'T' => Char::LatinCapitalLetterT, // Char 84 (('T', 1)) = LATIN CAPITAL LETTER T
            'U' => Char::LatinCapitalLetterU, // Char 85 (('U', 1)) = LATIN CAPITAL LETTER U
            'V' => Char::LatinCapitalLetterV, // Char 86 (('V', 1)) = LATIN CAPITAL LETTER V
            'W' => Char::LatinCapitalLetterW, // Char 87 (('W', 1)) = LATIN CAPITAL LETTER W
            'X' => Char::LatinCapitalLetterX, // Char 88 (('X', 1)) = LATIN CAPITAL LETTER X
            'Y' => Char::LatinCapitalLetterY, // Char 89 (('Y', 1)) = LATIN CAPITAL LETTER Y
            'Z' => Char::LatinCapitalLetterZ, // Char 90 (('Z', 1)) = LATIN CAPITAL LETTER Z
            '[' => Char::LeftSquareBracket, // Char 91 (('[', 1)) = LEFT SQUARE BRACKET
            '\\' => Char::ReverseSolidus, // Char 92 (('\\', 1)) = REVERSE SOLIDUS
            ']' => Char::RightSquareBracket, // Char 93 ((']', 1)) = RIGHT SQUARE BRACKET
            '^' => Char::CircumflexAccent, // Char 94 (('^', 1)) = CIRCUMFLEX ACCENT
            '_' => Char::LowLine,       // Char 95 (('_', 1)) = LOW LINE
            '`' => Char::GraveAccent,   // Char 96 (('`', 1)) = GRAVE ACCENT
            'a' => Char::LatinSmallLetterA, // Char 97 (('a', 1)) = LATIN SMALL LETTER A
            'b' => Char::LatinSmallLetterB, // Char 98 (('b', 1)) = LATIN SMALL LETTER B
            'c' => Char::LatinSmallLetterC, // Char 99 (('c', 1)) = LATIN SMALL LETTER C
            'd' => Char::LatinSmallLetterD, // Char 100 (('d', 1)) = LATIN SMALL LETTER D
            'e' => Char::LatinSmallLetterE, // Char 101 (('e', 1)) = LATIN SMALL LETTER E
            'f' => Char::LatinSmallLetterF, // Char 102 (('f', 1)) = LATIN SMALL LETTER F
            'g' => Char::LatinSmallLetterG, // Char 103 (('g', 1)) = LATIN SMALL LETTER G
            'h' => Char::LatinSmallLetterH, // Char 104 (('h', 1)) = LATIN SMALL LETTER H
            'i' => Char::LatinSmallLetterI, // Char 105 (('i', 1)) = LATIN SMALL LETTER I
            'j' => Char::LatinSmallLetterJ, // Char 106 (('j', 1)) = LATIN SMALL LETTER J
            'k' => Char::LatinSmallLetterK, // Char 107 (('k', 1)) = LATIN SMALL LETTER K
            'l' => Char::LatinSmallLetterL, // Char 108 (('l', 1)) = LATIN SMALL LETTER L
            'm' => Char::LatinSmallLetterM, // Char 109 (('m', 1)) = LATIN SMALL LETTER M
            'n' => Char::LatinSmallLetterN, // Char 110 (('n', 1)) = LATIN SMALL LETTER N
            'o' => Char::LatinSmallLetterO, // Char 111 (('o', 1)) = LATIN SMALL LETTER O
            'p' => Char::LatinSmallLetterP, // Char 112 (('p', 1)) = LATIN SMALL LETTER P
            'q' => Char::LatinSmallLetterQ, // Char 113 (('q', 1)) = LATIN SMALL LETTER Q
            'r' => Char::LatinSmallLetterR, // Char 114 (('r', 1)) = LATIN SMALL LETTER R
            's' => Char::LatinSmallLetterS, // Char 115 (('s', 1)) = LATIN SMALL LETTER S
            't' => Char::LatinSmallLetterT, // Char 116 (('t', 1)) = LATIN SMALL LETTER T
            'u' => Char::LatinSmallLetterU, // Char 117 (('u', 1)) = LATIN SMALL LETTER U
            'v' => Char::LatinSmallLetterV, // Char 118 (('v', 1)) = LATIN SMALL LETTER V
            'w' => Char::LatinSmallLetterW, // Char 119 (('w', 1)) = LATIN SMALL LETTER W
            'x' => Char::LatinSmallLetterX, // Char 120 (('x', 1)) = LATIN SMALL LETTER X
            'y' => Char::LatinSmallLetterY, // Char 121 (('y', 1)) = LATIN SMALL LETTER Y
            'z' => Char::LatinSmallLetterZ, // Char 122 (('z', 1)) = LATIN SMALL LETTER Z
            '{' => Char::LeftCurlyBracket, // Char 123 (('{', 1)) = LEFT CURLY BRACKET
            '|' => Char::VerticalLine,  // Char 124 (('|', 1)) = VERTICAL LINE
            '}' => Char::RightCurlyBracket, // Char 125 (('}', 1)) = RIGHT CURLY BRACKET
            '~' => Char::Tilde,         // Char 126 (('~', 1)) = TILDE
            '\u{007f}' => Char::Delete, // Char 127 (('\x7f', 1)) = DELETE
            'Ç' => Char::LatinCapitalLetterCWithCedilla, // Char 128 (('Ç', 1)) = LATIN CAPITAL LETTER C WITH CEDILLA
            'ü' => Char::LatinSmallLetterUWithDiaeresis, // Char 129 (('ü', 1)) = LATIN SMALL LETTER U WITH DIAERESIS
            'é' => Char::LatinSmallLetterEWithAcute, // Char 130 (('é', 1)) = LATIN SMALL LETTER E WITH ACUTE
            'â' => Char::LatinSmallLetterAWithCircumflex, // Char 131 (('â', 1)) = LATIN SMALL LETTER A WITH CIRCUMFLEX
            'ä' => Char::LatinSmallLetterAWithDiaeresis, // Char 132 (('ä', 1)) = LATIN SMALL LETTER A WITH DIAERESIS
            'à' => Char::LatinSmallLetterAWithGrave, // Char 133 (('à', 1)) = LATIN SMALL LETTER A WITH GRAVE
            'å' => Char::LatinSmallLetterAWithRingAbove, // Char 134 (('å', 1)) = LATIN SMALL LETTER A WITH RING ABOVE
            'ç' => Char::LatinSmallLetterCWithCedilla, // Char 135 (('ç', 1)) = LATIN SMALL LETTER C WITH CEDILLA
            'ê' => Char::LatinSmallLetterEWithCircumflex, // Char 136 (('ê', 1)) = LATIN SMALL LETTER E WITH CIRCUMFLEX
            'ë' => Char::LatinSmallLetterEWithDiaeresis, // Char 137 (('ë', 1)) = LATIN SMALL LETTER E WITH DIAERESIS
            'è' => Char::LatinSmallLetterEWithGrave, // Char 138 (('è', 1)) = LATIN SMALL LETTER E WITH GRAVE
            'ï' => Char::LatinSmallLetterIWithDiaeresis, // Char 139 (('ï', 1)) = LATIN SMALL LETTER I WITH DIAERESIS
            'î' => Char::LatinSmallLetterIWithCircumflex, // Char 140 (('î', 1)) = LATIN SMALL LETTER I WITH CIRCUMFLEX
            'ì' => Char::LatinSmallLetterIWithGrave, // Char 141 (('ì', 1)) = LATIN SMALL LETTER I WITH GRAVE
            'Ä' => Char::LatinCapitalLetterAWithDiaeresis, // Char 142 (('Ä', 1)) = LATIN CAPITAL LETTER A WITH DIAERESIS
            'Å' => Char::LatinCapitalLetterAWithRingAbove, // Char 143 (('Å', 1)) = LATIN CAPITAL LETTER A WITH RING ABOVE
            'É' => Char::LatinCapitalLetterEWithAcute, // Char 144 (('É', 1)) = LATIN CAPITAL LETTER E WITH ACUTE
            'æ' => Char::LatinSmallLetterAe, // Char 145 (('æ', 1)) = LATIN SMALL LETTER AE
            'Æ' => Char::LatinCapitalLetterAe, // Char 146 (('Æ', 1)) = LATIN CAPITAL LETTER AE
            'ô' => Char::LatinSmallLetterOWithCircumflex, // Char 147 (('ô', 1)) = LATIN SMALL LETTER O WITH CIRCUMFLEX
            'ö' => Char::LatinSmallLetterOWithDiaeresis, // Char 148 (('ö', 1)) = LATIN SMALL LETTER O WITH DIAERESIS
            'ò' => Char::LatinSmallLetterOWithGrave, // Char 149 (('ò', 1)) = LATIN SMALL LETTER O WITH GRAVE
            'û' => Char::LatinSmallLetterUWithCircumflex, // Char 150 (('û', 1)) = LATIN SMALL LETTER U WITH CIRCUMFLEX
            'ù' => Char::LatinSmallLetterUWithGrave, // Char 151 (('ù', 1)) = LATIN SMALL LETTER U WITH GRAVE
            'ÿ' => Char::LatinSmallLetterYWithDiaeresis, // Char 152 (('ÿ', 1)) = LATIN SMALL LETTER Y WITH DIAERESIS
            'Ö' => Char::LatinCapitalLetterOWithDiaeresis, // Char 153 (('Ö', 1)) = LATIN CAPITAL LETTER O WITH DIAERESIS
            'Ü' => Char::LatinCapitalLetterUWithDiaeresis, // Char 154 (('Ü', 1)) = LATIN CAPITAL LETTER U WITH DIAERESIS
            'ø' => Char::LatinSmallLetterOWithStroke, // Char 155 (('ø', 1)) = LATIN SMALL LETTER O WITH STROKE
            '£' => Char::PoundSign,                   // Char 156 (('£', 1)) = POUND SIGN
            'Ø' => Char::LatinCapitalLetterOWithStroke, // Char 157 (('Ø', 1)) = LATIN CAPITAL LETTER O WITH STROKE
            '×' => Char::MultiplicationSign, // Char 158 (('×', 1)) = MULTIPLICATION SIGN
            'ƒ' => Char::LatinSmallLetterFWithHook, // Char 159 (('ƒ', 1)) = LATIN SMALL LETTER F WITH HOOK
            'á' => Char::LatinSmallLetterAWithAcute, // Char 160 (('á', 1)) = LATIN SMALL LETTER A WITH ACUTE
            'í' => Char::LatinSmallLetterIWithAcute, // Char 161 (('í', 1)) = LATIN SMALL LETTER I WITH ACUTE
            'ó' => Char::LatinSmallLetterOWithAcute, // Char 162 (('ó', 1)) = LATIN SMALL LETTER O WITH ACUTE
            'ú' => Char::LatinSmallLetterUWithAcute, // Char 163 (('ú', 1)) = LATIN SMALL LETTER U WITH ACUTE
            'ñ' => Char::LatinSmallLetterNWithTilde, // Char 164 (('ñ', 1)) = LATIN SMALL LETTER N WITH TILDE
            'Ñ' => Char::LatinCapitalLetterNWithTilde, // Char 165 (('Ñ', 1)) = LATIN CAPITAL LETTER N WITH TILDE
            'ª' => Char::FeminineOrdinalIndicator, // Char 166 (('ª', 1)) = FEMININE ORDINAL INDICATOR
            'º' => Char::MasculineOrdinalIndicator, // Char 167 (('º', 1)) = MASCULINE ORDINAL INDICATOR
            '¿' => Char::InvertedQuestionMark, // Char 168 (('¿', 1)) = INVERTED QUESTION MARK
            '®' => Char::RegisteredSign,       // Char 169 (('®', 1)) = REGISTERED SIGN
            '¬' => Char::NotSign,              // Char 170 (('¬', 1)) = NOT SIGN
            '½' => Char::VulgarFractionOneHalf, // Char 171 (('½', 1)) = VULGAR FRACTION ONE HALF
            '¼' => Char::VulgarFractionOneQuarter, // Char 172 (('¼', 1)) = VULGAR FRACTION ONE QUARTER
            '¡' => Char::InvertedExclamationMark, // Char 173 (('¡', 1)) = INVERTED EXCLAMATION MARK
            '«' => Char::LeftPointingDoubleAngleQuotationMark, // Char 174 (('«', 1)) = LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
            '»' => Char::RightPointingDoubleAngleQuotationMark, // Char 175 (('»', 1)) = RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
            '░' => Char::LightShade, // Char 176 (('░', 1)) = LIGHT SHADE
            '▒' => Char::MediumShade, // Char 177 (('▒', 1)) = MEDIUM SHADE
            '▓' => Char::DarkShade,  // Char 178 (('▓', 1)) = DARK SHADE
            '│' => Char::BoxDrawingsLightVertical, // Char 179 (('│', 1)) = BOX DRAWINGS LIGHT VERTICAL
            '┤' => Char::BoxDrawingsLightVerticalAndLeft, // Char 180 (('┤', 1)) = BOX DRAWINGS LIGHT VERTICAL AND LEFT
            'Á' => Char::LatinCapitalLetterAWithAcute, // Char 181 (('Á', 1)) = LATIN CAPITAL LETTER A WITH ACUTE
            'Â' => Char::LatinCapitalLetterAWithCircumflex, // Char 182 (('Â', 1)) = LATIN CAPITAL LETTER A WITH CIRCUMFLEX
            'À' => Char::LatinCapitalLetterAWithGrave, // Char 183 (('À', 1)) = LATIN CAPITAL LETTER A WITH GRAVE
            '©' => Char::CopyrightSign,                // Char 184 (('©', 1)) = COPYRIGHT SIGN
            '╣' => Char::BoxDrawingsDoubleVerticalAndLeft, // Char 185 (('╣', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND LEFT
            '║' => Char::BoxDrawingsDoubleVertical, // Char 186 (('║', 1)) = BOX DRAWINGS DOUBLE VERTICAL
            '╗' => Char::BoxDrawingsDoubleDownAndLeft, // Char 187 (('╗', 1)) = BOX DRAWINGS DOUBLE DOWN AND LEFT
            '╝' => Char::BoxDrawingsDoubleUpAndLeft, // Char 188 (('╝', 1)) = BOX DRAWINGS DOUBLE UP AND LEFT
            '¢' => Char::CentSign,                    // Char 189 (('¢', 1)) = CENT SIGN
            '¥' => Char::YenSign,                     // Char 190 (('¥', 1)) = YEN SIGN
            '┐' => Char::BoxDrawingsLightDownAndLeft, // Char 191 (('┐', 1)) = BOX DRAWINGS LIGHT DOWN AND LEFT
            '└' => Char::BoxDrawingsLightUpAndRight, // Char 192 (('└', 1)) = BOX DRAWINGS LIGHT UP AND RIGHT
            '┴' => Char::BoxDrawingsLightUpAndHorizontal, // Char 193 (('┴', 1)) = BOX DRAWINGS LIGHT UP AND HORIZONTAL
            '┬' => Char::BoxDrawingsLightDownAndHorizontal, // Char 194 (('┬', 1)) = BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
            '├' => Char::BoxDrawingsLightVerticalAndRight, // Char 195 (('├', 1)) = BOX DRAWINGS LIGHT VERTICAL AND RIGHT
            '─' => Char::BoxDrawingsLightHorizontal, // Char 196 (('─', 1)) = BOX DRAWINGS LIGHT HORIZONTAL
            '┼' => Char::BoxDrawingsLightVerticalAndHorizontal, // Char 197 (('┼', 1)) = BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
            'ã' => Char::LatinSmallLetterAWithTilde, // Char 198 (('ã', 1)) = LATIN SMALL LETTER A WITH TILDE
            'Ã' => Char::LatinCapitalLetterAWithTilde, // Char 199 (('Ã', 1)) = LATIN CAPITAL LETTER A WITH TILDE
            '╚' => Char::BoxDrawingsDoubleUpAndRight, // Char 200 (('╚', 1)) = BOX DRAWINGS DOUBLE UP AND RIGHT
            '╔' => Char::BoxDrawingsDoubleDownAndRight, // Char 201 (('╔', 1)) = BOX DRAWINGS DOUBLE DOWN AND RIGHT
            '╩' => Char::BoxDrawingsDoubleUpAndHorizontal, // Char 202 (('╩', 1)) = BOX DRAWINGS DOUBLE UP AND HORIZONTAL
            '╦' => Char::BoxDrawingsDoubleDownAndHorizontal, // Char 203 (('╦', 1)) = BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
            '╠' => Char::BoxDrawingsDoubleVerticalAndRight, // Char 204 (('╠', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
            '═' => Char::BoxDrawingsDoubleHorizontal, // Char 205 (('═', 1)) = BOX DRAWINGS DOUBLE HORIZONTAL
            '╬' => Char::BoxDrawingsDoubleVerticalAndHorizontal, // Char 206 (('╬', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
            '¤' => Char::CurrencySign, // Char 207 (('¤', 1)) = CURRENCY SIGN
            'ð' => Char::LatinSmallLetterEth, // Char 208 (('ð', 1)) = LATIN SMALL LETTER ETH
            'Ð' => Char::LatinCapitalLetterEth, // Char 209 (('Ð', 1)) = LATIN CAPITAL LETTER ETH
            'Ê' => Char::LatinCapitalLetterEWithCircumflex, // Char 210 (('Ê', 1)) = LATIN CAPITAL LETTER E WITH CIRCUMFLEX
            'Ë' => Char::LatinCapitalLetterEWithDiaeresis, // Char 211 (('Ë', 1)) = LATIN CAPITAL LETTER E WITH DIAERESIS
            'È' => Char::LatinCapitalLetterEWithGrave, // Char 212 (('È', 1)) = LATIN CAPITAL LETTER E WITH GRAVE
            'ı' => Char::LatinSmallLetterDotlessI, // Char 213 (('ı', 1)) = LATIN SMALL LETTER DOTLESS I
            'Í' => Char::LatinCapitalLetterIWithAcute, // Char 214 (('Í', 1)) = LATIN CAPITAL LETTER I WITH ACUTE
            'Î' => Char::LatinCapitalLetterIWithCircumflex, // Char 215 (('Î', 1)) = LATIN CAPITAL LETTER I WITH CIRCUMFLEX
            'Ï' => Char::LatinCapitalLetterIWithDiaeresis, // Char 216 (('Ï', 1)) = LATIN CAPITAL LETTER I WITH DIAERESIS
            '┘' => Char::BoxDrawingsLightUpAndLeft, // Char 217 (('┘', 1)) = BOX DRAWINGS LIGHT UP AND LEFT
            '┌' => Char::BoxDrawingsLightDownAndRight, // Char 218 (('┌', 1)) = BOX DRAWINGS LIGHT DOWN AND RIGHT
            '█' => Char::FullBlock,                    // Char 219 (('█', 1)) = FULL BLOCK
            '▄' => Char::LowerHalfBlock, // Char 220 (('▄', 1)) = LOWER HALF BLOCK
            '¦' => Char::BrokenBar,       // Char 221 (('¦', 1)) = BROKEN BAR
            'Ì' => Char::LatinCapitalLetterIWithGrave, // Char 222 (('Ì', 1)) = LATIN CAPITAL LETTER I WITH GRAVE
            '▀' => Char::UpperHalfBlock, // Char 223 (('▀', 1)) = UPPER HALF BLOCK
            'Ó' => Char::LatinCapitalLetterOWithAcute, // Char 224 (('Ó', 1)) = LATIN CAPITAL LETTER O WITH ACUTE
            'ß' => Char::LatinSmallLetterSharpS, // Char 225 (('ß', 1)) = LATIN SMALL LETTER SHARP S
            'Ô' => Char::LatinCapitalLetterOWithCircumflex, // Char 226 (('Ô', 1)) = LATIN CAPITAL LETTER O WITH CIRCUMFLEX
            'Ò' => Char::LatinCapitalLetterOWithGrave, // Char 227 (('Ò', 1)) = LATIN CAPITAL LETTER O WITH GRAVE
            'õ' => Char::LatinSmallLetterOWithTilde, // Char 228 (('õ', 1)) = LATIN SMALL LETTER O WITH TILDE
            'Õ' => Char::LatinCapitalLetterOWithTilde, // Char 229 (('Õ', 1)) = LATIN CAPITAL LETTER O WITH TILDE
            'µ' => Char::MicroSign,                    // Char 230 (('µ', 1)) = MICRO SIGN
            'þ' => Char::LatinSmallLetterThorn, // Char 231 (('þ', 1)) = LATIN SMALL LETTER THORN
            'Þ' => Char::LatinCapitalLetterThorn, // Char 232 (('Þ', 1)) = LATIN CAPITAL LETTER THORN
            'Ú' => Char::LatinCapitalLetterUWithAcute, // Char 233 (('Ú', 1)) = LATIN CAPITAL LETTER U WITH ACUTE
            'Û' => Char::LatinCapitalLetterUWithCircumflex, // Char 234 (('Û', 1)) = LATIN CAPITAL LETTER U WITH CIRCUMFLEX
            'Ù' => Char::LatinCapitalLetterUWithGrave, // Char 235 (('Ù', 1)) = LATIN CAPITAL LETTER U WITH GRAVE
            'ý' => Char::LatinSmallLetterYWithAcute, // Char 236 (('ý', 1)) = LATIN SMALL LETTER Y WITH ACUTE
            'Ý' => Char::LatinCapitalLetterYWithAcute, // Char 237 (('Ý', 1)) = LATIN CAPITAL LETTER Y WITH ACUTE
            '¯' => Char::Macron,                       // Char 238 (('¯', 1)) = MACRON
            '´' => Char::AcuteAccent,                  // Char 239 (('´', 1)) = ACUTE ACCENT
            '\u{00ad}' => Char::SoftHyphen,             // Char 240 (('\xad', 1)) = SOFT HYPHEN
            '±' => Char::PlusMinusSign,                // Char 241 (('±', 1)) = PLUS-MINUS SIGN
            '‗' => Char::DoubleLowLine,               // Char 242 (('‗', 1)) = DOUBLE LOW LINE
            '¾' => Char::VulgarFractionThreeQuarters, // Char 243 (('¾', 1)) = VULGAR FRACTION THREE QUARTERS
            '¶' => Char::PilcrowSign,                 // Char 244 (('¶', 1)) = PILCROW SIGN
            '§' => Char::SectionSign,                 // Char 245 (('§', 1)) = SECTION SIGN
            '÷' => Char::DivisionSign,                // Char 246 (('÷', 1)) = DIVISION SIGN
            '¸' => Char::Cedilla,                     // Char 247 (('¸', 1)) = CEDILLA
            '°' => Char::DegreeSign,                  // Char 248 (('°', 1)) = DEGREE SIGN
            '¨' => Char::Diaeresis,                   // Char 249 (('¨', 1)) = DIAERESIS
            '·' => Char::MiddleDot,                   // Char 250 (('·', 1)) = MIDDLE DOT
            '¹' => Char::SuperscriptOne,              // Char 251 (('¹', 1)) = SUPERSCRIPT ONE
            '³' => Char::SuperscriptThree,            // Char 252 (('³', 1)) = SUPERSCRIPT THREE
            '²' => Char::SuperscriptTwo,              // Char 253 (('²', 1)) = SUPERSCRIPT TWO
            '■' => Char::BlackSquare,                // Char 254 (('■', 1)) = BLACK SQUARE
            '\u{00a0}' => Char::NoBreakSpace,          // Char 255 (('\xa0', 1)) = NO-BREAK SPACE
            '☺' => Char::SOH, // Char 001 (('☺', 1)) = WHITE SMILING FACE
            '☻' => Char::STX, // Char 002 (('☻', 1)) = BLACK SMILING FACE
            '♥' => Char::ETX, // Char 003 (('♥', 1)) = BLACK HEART SUIT
            '♦' => Char::EOT, // Char 004 (('♦', 1)) = BLACK DIAMOND SUIT
            '♣' => Char::ENQ, // Char 005 (('♣', 1)) = BLACK CLUB SUIT
            '♠' => Char::ACK, // Char 006 (('♠', 1)) = BLACK SPADE SUIT
            '•' => Char::BEL, // Char 007 (('•', 1)) = BULLET
            '◘' => Char::BS,  // Char 008 (('◘', 1)) = INVERSE BULLET
            '○' => Char::HT,  // Char 009 (('○', 1)) = WHITE CIRCLE
            '◙' => Char::LF,  // Char 010 (('◙', 1)) = INVERSE WHITE CIRCLE
            '♂' => Char::VT,  // Char 011 (('♂', 1)) = MALE SIGN
            '♀' => Char::FF,  // Char 012 (('♀', 1)) = FEMALE SIGN
            '♪' => Char::CR,  // Char 013 (('♪', 1)) = EIGHTH NOTE
            '♫' => Char::SO,  // Char 014 (('♫', 1)) = BEAMED EIGHTH NOTES
            '☼' => Char::SI,  // Char 015 (('☼', 1)) = WHITE SUN WITH RAYS
            '►' => Char::DLE, // Char 016 (('►', 1)) = BLACK RIGHT-POINTING ARROW
            '◄' => Char::DC1, // Char 017 (('◄', 1)) = BLACK LEFT-POINTING ARROW
            '↕' => Char::DC2, // Char 018 (('↕', 1)) = UP DOWN ARROW
            '‼' => Char::DC3, // Char 019 (('‼', 1)) = DOUBLE EXCLAMATION MARK
            '▬' => Char::SYN, // Char 022 (('▬', 1)) = BLACK RECTANGLE
            '↨' => Char::ETB, // Char 023 (('↨', 1)) = UP DOWN ARROW WITH BASE
            '↑' => Char::CAN, // Char 024 (('↑', 1)) = UPWARDS ARROW
            '↓' => Char::EM,  // Char 025 (('↓', 1)) = DOWNWARDS ARROW
            '→' => Char::SUB, // Char 026 (('→', 1)) = RIGHTWARDS ARROW
            '←' => Char::Escape, // Char 027 (('←', 1)) = LEFTWARDS ARROW
            '∟' => Char::FS,  // Char 028 (('∟', 1)) = RIGHT ANDLE
            '↔' => Char::GS,  // Char 029 (('↔', 1)) = LEFT RIGHT ARROW
            '▲' => Char::RS,  // Char 030 (('▲', 1)) = BLACK UP POINTING ARROW
            '▼' => Char::US,  // Char 031 (('▼', 1)) = BLACK DOWN POINTING ARROW
            _ => Char::QuestionMark, // Map unknown chars to ?
        }
    }

    /// Take a CodePage 850 byte
    pub fn from_byte(byte: u8) -> Char {
        unsafe { ::core::mem::transmute(byte) }
    }
}

// End of file
