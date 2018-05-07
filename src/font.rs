pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 16;

/// This is our own custom character set.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Glyph {
    Null,                                   // Glyph 000 ((' ', 1)) = REPLACEMENT CHARACTER
    SOH,                                    // Glyph 001 (('☺', 1)) = WHITE SMILING FACE
    STX,                                    // Glyph 002 (('☻', 1)) = BLACK SMILING FACE
    ETX,                                    // Glyph 003 (('♥', 1)) = BLACK HEART SUIT
    EOT,                                    // Glyph 004 (('♦', 1)) = BLACK DIAMOND SUIT
    ENQ,                                    // Glyph 005 (('♣', 1)) = BLACK CLUB SUIT
    ACK,                                    // Glyph 006 (('♠', 1)) = BLACK SPADE SUIT
    BEL,                                    // Glyph 007 (('•', 1)) = BULLET
    BS,                                     // Glyph 008 (('◘', 1)) = INVERSE BULLET
    HT,                                     // Glyph 009 (('○', 1)) = WHITE CIRCLE
    LF,                                     // Glyph 010 (('◙', 1)) = INVERSE WHITE CIRCLE
    VT,                                     // Glyph 011 (('♂', 1)) = MALE SIGN
    FF,                                     // Glyph 012 (('♀', 1)) = FEMALE SIGN
    CR,                                     // Glyph 013 (('♪', 1)) = EIGHTH NOTE
    SO,                                     // Glyph 014 (('♫', 1)) = BEAMED EIGHTH NOTES
    SI,                                     // Glyph 015 (('☼', 1)) = WHITE SUN WITH RAYS
    DLE,                                    // Glyph 016 (('►', 1)) = BLACK RIGHT-POINTING ARROW
    DC1,                                    // Glyph 017 (('◄', 1)) = BLACK LEFT-POINTING ARROW
    DC2,                                    // Glyph 018 (('↕', 1)) = UP DOWN ARROW
    DC3,                                    // Glyph 019 (('‼', 1)) = DOUBLE EXCLAMATION MARK
    DC4,                                    // Glyph 020 (('¶', 1)) = PILCROW SIGN
    NAK,                                    // Glyph 021 (('§', 1)) = SECTION SIGN
    SYN,                                    // Glyph 022 (('▬', 1)) = BLACK RECTANGLE
    ETB,                                    // Glyph 023 (('↨', 1)) = UP DOWN ARROW WITH BASE
    CAN,                                    // Glyph 024 (('↑', 1)) = UPWARDS ARROW
    EM,                                     // Glyph 025 (('↓', 1)) = DOWNWARDS ARROW
    SUB,                                    // Glyph 026 (('→', 1)) = RIGHTWARDS ARROW
    Escape,                                 // Glyph 027 (('←', 1)) = LEFTWARDS ARROW
    FS,                                     // Glyph 028 (('∟', 1)) = RIGHT ANDLE
    GS,                                     // Glyph 029 (('↔', 1)) = LEFT RIGHT ARROW
    RS,                                     // Glyph 030 (('▲', 1)) = BLACK UP POINTING ARROW
    US,                                     // Glyph 031 (('▼', 1)) = BLACK DOWN POINTING ARROW
    Space,                                  // Glyph 032 ((' ', 1)) = SPACE
    ExclamationMark,                        // Glyph 033 (('!', 1)) = EXCLAMATION MARK
    QuotationMark,                          // Glyph 034 (('"', 1)) = QUOTATION MARK
    NumberSign,                             // Glyph 035 (('#', 1)) = NUMBER SIGN
    DollarSign,                             // Glyph 036 (('$', 1)) = DOLLAR SIGN
    PercentSign,                            // Glyph 037 (('%', 1)) = PERCENT SIGN
    Ampersand,                              // Glyph 038 (('&', 1)) = AMPERSAND
    Apostrophe,                             // Glyph 039 (("'", 1)) = APOSTROPHE
    LeftParenthesis,                        // Glyph 040 (('(', 1)) = LEFT PARENTHESIS
    RightParenthesis,                       // Glyph 041 ((')', 1)) = RIGHT PARENTHESIS
    Asterisk,                               // Glyph 042 (('*', 1)) = ASTERISK
    PlusSign,                               // Glyph 043 (('+', 1)) = PLUS SIGN
    Comma,                                  // Glyph 044 ((',', 1)) = COMMA
    HyphenMinus,                            // Glyph 045 (('-', 1)) = HYPHEN-MINUS
    FullStop,                               // Glyph 046 (('.', 1)) = FULL STOP
    Solidus,                                // Glyph 047 (('/', 1)) = SOLIDUS
    DigitZero,                              // Glyph 048 (('0', 1)) = DIGIT ZERO
    DigitOne,                               // Glyph 049 (('1', 1)) = DIGIT ONE
    DigitTwo,                               // Glyph 050 (('2', 1)) = DIGIT TWO
    DigitThree,                             // Glyph 051 (('3', 1)) = DIGIT THREE
    DigitFour,                              // Glyph 052 (('4', 1)) = DIGIT FOUR
    DigitFive,                              // Glyph 053 (('5', 1)) = DIGIT FIVE
    DigitSix,                               // Glyph 054 (('6', 1)) = DIGIT SIX
    DigitSeven,                             // Glyph 055 (('7', 1)) = DIGIT SEVEN
    DigitEight,                             // Glyph 056 (('8', 1)) = DIGIT EIGHT
    DigitNine,                              // Glyph 057 (('9', 1)) = DIGIT NINE
    Colon,                                  // Glyph 058 ((':', 1)) = COLON
    Semicolon,                              // Glyph 059 ((';', 1)) = SEMICOLON
    LessThanSign,                           // Glyph 060 (('<', 1)) = LESS-THAN SIGN
    EqualsSign,                             // Glyph 061 (('=', 1)) = EQUALS SIGN
    GreaterThanSign,                        // Glyph 062 (('>', 1)) = GREATER-THAN SIGN
    QuestionMark,                           // Glyph 063 (('?', 1)) = QUESTION MARK
    CommercialAt,                           // Glyph 064 (('@', 1)) = COMMERCIAL AT
    LatinCapitalLetterA,                    // Glyph 065 (('A', 1)) = LATIN CAPITAL LETTER A
    LatinCapitalLetterB,                    // Glyph 066 (('B', 1)) = LATIN CAPITAL LETTER B
    LatinCapitalLetterC,                    // Glyph 067 (('C', 1)) = LATIN CAPITAL LETTER C
    LatinCapitalLetterD,                    // Glyph 068 (('D', 1)) = LATIN CAPITAL LETTER D
    LatinCapitalLetterE,                    // Glyph 069 (('E', 1)) = LATIN CAPITAL LETTER E
    LatinCapitalLetterF,                    // Glyph 070 (('F', 1)) = LATIN CAPITAL LETTER F
    LatinCapitalLetterG,                    // Glyph 071 (('G', 1)) = LATIN CAPITAL LETTER G
    LatinCapitalLetterH,                    // Glyph 072 (('H', 1)) = LATIN CAPITAL LETTER H
    LatinCapitalLetterI,                    // Glyph 073 (('I', 1)) = LATIN CAPITAL LETTER I
    LatinCapitalLetterJ,                    // Glyph 074 (('J', 1)) = LATIN CAPITAL LETTER J
    LatinCapitalLetterK,                    // Glyph 075 (('K', 1)) = LATIN CAPITAL LETTER K
    LatinCapitalLetterL,                    // Glyph 076 (('L', 1)) = LATIN CAPITAL LETTER L
    LatinCapitalLetterM,                    // Glyph 077 (('M', 1)) = LATIN CAPITAL LETTER M
    LatinCapitalLetterN,                    // Glyph 078 (('N', 1)) = LATIN CAPITAL LETTER N
    LatinCapitalLetterO,                    // Glyph 079 (('O', 1)) = LATIN CAPITAL LETTER O
    LatinCapitalLetterP,                    // Glyph 080 (('P', 1)) = LATIN CAPITAL LETTER P
    LatinCapitalLetterQ,                    // Glyph 081 (('Q', 1)) = LATIN CAPITAL LETTER Q
    LatinCapitalLetterR,                    // Glyph 082 (('R', 1)) = LATIN CAPITAL LETTER R
    LatinCapitalLetterS,                    // Glyph 083 (('S', 1)) = LATIN CAPITAL LETTER S
    LatinCapitalLetterT,                    // Glyph 084 (('T', 1)) = LATIN CAPITAL LETTER T
    LatinCapitalLetterU,                    // Glyph 085 (('U', 1)) = LATIN CAPITAL LETTER U
    LatinCapitalLetterV,                    // Glyph 086 (('V', 1)) = LATIN CAPITAL LETTER V
    LatinCapitalLetterW,                    // Glyph 087 (('W', 1)) = LATIN CAPITAL LETTER W
    LatinCapitalLetterX,                    // Glyph 088 (('X', 1)) = LATIN CAPITAL LETTER X
    LatinCapitalLetterY,                    // Glyph 089 (('Y', 1)) = LATIN CAPITAL LETTER Y
    LatinCapitalLetterZ,                    // Glyph 090 (('Z', 1)) = LATIN CAPITAL LETTER Z
    LeftSquareBracket,                      // Glyph 091 (('[', 1)) = LEFT SQUARE BRACKET
    ReverseSolidus,                         // Glyph 092 (('\\', 1)) = REVERSE SOLIDUS
    RightSquareBracket,                     // Glyph 093 ((']', 1)) = RIGHT SQUARE BRACKET
    CircumflexAccent,                       // Glyph 094 (('^', 1)) = CIRCUMFLEX ACCENT
    LowLine,                                // Glyph 095 (('_', 1)) = LOW LINE
    GraveAccent,                            // Glyph 096 (('`', 1)) = GRAVE ACCENT
    LatinSmallLetterA,                      // Glyph 097 (('a', 1)) = LATIN SMALL LETTER A
    LatinSmallLetterB,                      // Glyph 098 (('b', 1)) = LATIN SMALL LETTER B
    LatinSmallLetterC,                      // Glyph 099 (('c', 1)) = LATIN SMALL LETTER C
    LatinSmallLetterD,                      // Glyph 100 (('d', 1)) = LATIN SMALL LETTER D
    LatinSmallLetterE,                      // Glyph 101 (('e', 1)) = LATIN SMALL LETTER E
    LatinSmallLetterF,                      // Glyph 102 (('f', 1)) = LATIN SMALL LETTER F
    LatinSmallLetterG,                      // Glyph 103 (('g', 1)) = LATIN SMALL LETTER G
    LatinSmallLetterH,                      // Glyph 104 (('h', 1)) = LATIN SMALL LETTER H
    LatinSmallLetterI,                      // Glyph 105 (('i', 1)) = LATIN SMALL LETTER I
    LatinSmallLetterJ,                      // Glyph 106 (('j', 1)) = LATIN SMALL LETTER J
    LatinSmallLetterK,                      // Glyph 107 (('k', 1)) = LATIN SMALL LETTER K
    LatinSmallLetterL,                      // Glyph 108 (('l', 1)) = LATIN SMALL LETTER L
    LatinSmallLetterM,                      // Glyph 109 (('m', 1)) = LATIN SMALL LETTER M
    LatinSmallLetterN,                      // Glyph 110 (('n', 1)) = LATIN SMALL LETTER N
    LatinSmallLetterO,                      // Glyph 111 (('o', 1)) = LATIN SMALL LETTER O
    LatinSmallLetterP,                      // Glyph 112 (('p', 1)) = LATIN SMALL LETTER P
    LatinSmallLetterQ,                      // Glyph 113 (('q', 1)) = LATIN SMALL LETTER Q
    LatinSmallLetterR,                      // Glyph 114 (('r', 1)) = LATIN SMALL LETTER R
    LatinSmallLetterS,                      // Glyph 115 (('s', 1)) = LATIN SMALL LETTER S
    LatinSmallLetterT,                      // Glyph 116 (('t', 1)) = LATIN SMALL LETTER T
    LatinSmallLetterU,                      // Glyph 117 (('u', 1)) = LATIN SMALL LETTER U
    LatinSmallLetterV,                      // Glyph 118 (('v', 1)) = LATIN SMALL LETTER V
    LatinSmallLetterW,                      // Glyph 119 (('w', 1)) = LATIN SMALL LETTER W
    LatinSmallLetterX,                      // Glyph 120 (('x', 1)) = LATIN SMALL LETTER X
    LatinSmallLetterY,                      // Glyph 121 (('y', 1)) = LATIN SMALL LETTER Y
    LatinSmallLetterZ,                      // Glyph 122 (('z', 1)) = LATIN SMALL LETTER Z
    LeftCurlyBracket,                       // Glyph 123 (('{', 1)) = LEFT CURLY BRACKET
    VerticalLine,                           // Glyph 124 (('|', 1)) = VERTICAL LINE
    RightCurlyBracket,                      // Glyph 125 (('}', 1)) = RIGHT CURLY BRACKET
    Tilde,                                  // Glyph 126 (('~', 1)) = TILDE
    Delete,                                 // Glyph 127 (('\x7f', 1)) = DELETE
    LatinCapitalLetterCWithCedilla,         // Glyph 128 (('Ç', 1)) = LATIN CAPITAL LETTER C WITH CEDILLA
    LatinSmallLetterUWithDiaeresis,         // Glyph 129 (('ü', 1)) = LATIN SMALL LETTER U WITH DIAERESIS
    LatinSmallLetterEWithAcute,             // Glyph 130 (('é', 1)) = LATIN SMALL LETTER E WITH ACUTE
    LatinSmallLetterAWithCircumflex,        // Glyph 131 (('â', 1)) = LATIN SMALL LETTER A WITH CIRCUMFLEX
    LatinSmallLetterAWithDiaeresis,         // Glyph 132 (('ä', 1)) = LATIN SMALL LETTER A WITH DIAERESIS
    LatinSmallLetterAWithGrave,             // Glyph 133 (('à', 1)) = LATIN SMALL LETTER A WITH GRAVE
    LatinSmallLetterAWithRingAbove,         // Glyph 134 (('å', 1)) = LATIN SMALL LETTER A WITH RING ABOVE
    LatinSmallLetterCWithCedilla,           // Glyph 135 (('ç', 1)) = LATIN SMALL LETTER C WITH CEDILLA
    LatinSmallLetterEWithCircumflex,        // Glyph 136 (('ê', 1)) = LATIN SMALL LETTER E WITH CIRCUMFLEX
    LatinSmallLetterEWithDiaeresis,         // Glyph 137 (('ë', 1)) = LATIN SMALL LETTER E WITH DIAERESIS
    LatinSmallLetterEWithGrave,             // Glyph 138 (('è', 1)) = LATIN SMALL LETTER E WITH GRAVE
    LatinSmallLetterIWithDiaeresis,         // Glyph 139 (('ï', 1)) = LATIN SMALL LETTER I WITH DIAERESIS
    LatinSmallLetterIWithCircumflex,        // Glyph 140 (('î', 1)) = LATIN SMALL LETTER I WITH CIRCUMFLEX
    LatinSmallLetterIWithGrave,             // Glyph 141 (('ì', 1)) = LATIN SMALL LETTER I WITH GRAVE
    LatinCapitalLetterAWithDiaeresis,       // Glyph 142 (('Ä', 1)) = LATIN CAPITAL LETTER A WITH DIAERESIS
    LatinCapitalLetterAWithRingAbove,       // Glyph 143 (('Å', 1)) = LATIN CAPITAL LETTER A WITH RING ABOVE
    LatinCapitalLetterEWithAcute,           // Glyph 144 (('É', 1)) = LATIN CAPITAL LETTER E WITH ACUTE
    LatinSmallLetterAe,                     // Glyph 145 (('æ', 1)) = LATIN SMALL LETTER AE
    LatinCapitalLetterAe,                   // Glyph 146 (('Æ', 1)) = LATIN CAPITAL LETTER AE
    LatinSmallLetterOWithCircumflex,        // Glyph 147 (('ô', 1)) = LATIN SMALL LETTER O WITH CIRCUMFLEX
    LatinSmallLetterOWithDiaeresis,         // Glyph 148 (('ö', 1)) = LATIN SMALL LETTER O WITH DIAERESIS
    LatinSmallLetterOWithGrave,             // Glyph 149 (('ò', 1)) = LATIN SMALL LETTER O WITH GRAVE
    LatinSmallLetterUWithCircumflex,        // Glyph 150 (('û', 1)) = LATIN SMALL LETTER U WITH CIRCUMFLEX
    LatinSmallLetterUWithGrave,             // Glyph 151 (('ù', 1)) = LATIN SMALL LETTER U WITH GRAVE
    LatinSmallLetterYWithDiaeresis,         // Glyph 152 (('ÿ', 1)) = LATIN SMALL LETTER Y WITH DIAERESIS
    LatinCapitalLetterOWithDiaeresis,       // Glyph 153 (('Ö', 1)) = LATIN CAPITAL LETTER O WITH DIAERESIS
    LatinCapitalLetterUWithDiaeresis,       // Glyph 154 (('Ü', 1)) = LATIN CAPITAL LETTER U WITH DIAERESIS
    LatinSmallLetterOWithStroke,            // Glyph 155 (('ø', 1)) = LATIN SMALL LETTER O WITH STROKE
    PoundSign,                              // Glyph 156 (('£', 1)) = POUND SIGN
    LatinCapitalLetterOWithStroke,          // Glyph 157 (('Ø', 1)) = LATIN CAPITAL LETTER O WITH STROKE
    MultiplicationSign,                     // Glyph 158 (('×', 1)) = MULTIPLICATION SIGN
    LatinSmallLetterFWithHook,              // Glyph 159 (('ƒ', 1)) = LATIN SMALL LETTER F WITH HOOK
    LatinSmallLetterAWithAcute,             // Glyph 160 (('á', 1)) = LATIN SMALL LETTER A WITH ACUTE
    LatinSmallLetterIWithAcute,             // Glyph 161 (('í', 1)) = LATIN SMALL LETTER I WITH ACUTE
    LatinSmallLetterOWithAcute,             // Glyph 162 (('ó', 1)) = LATIN SMALL LETTER O WITH ACUTE
    LatinSmallLetterUWithAcute,             // Glyph 163 (('ú', 1)) = LATIN SMALL LETTER U WITH ACUTE
    LatinSmallLetterNWithTilde,             // Glyph 164 (('ñ', 1)) = LATIN SMALL LETTER N WITH TILDE
    LatinCapitalLetterNWithTilde,           // Glyph 165 (('Ñ', 1)) = LATIN CAPITAL LETTER N WITH TILDE
    FeminineOrdinalIndicator,               // Glyph 166 (('ª', 1)) = FEMININE ORDINAL INDICATOR
    MasculineOrdinalIndicator,              // Glyph 167 (('º', 1)) = MASCULINE ORDINAL INDICATOR
    InvertedQuestionMark,                   // Glyph 168 (('¿', 1)) = INVERTED QUESTION MARK
    RegisteredSign,                         // Glyph 169 (('®', 1)) = REGISTERED SIGN
    NotSign,                                // Glyph 170 (('¬', 1)) = NOT SIGN
    VulgarFractionOneHalf,                  // Glyph 171 (('½', 1)) = VULGAR FRACTION ONE HALF
    VulgarFractionOneQuarter,               // Glyph 172 (('¼', 1)) = VULGAR FRACTION ONE QUARTER
    InvertedExclamationMark,                // Glyph 173 (('¡', 1)) = INVERTED EXCLAMATION MARK
    LeftPointingDoubleAngleQuotationMark,   // Glyph 174 (('«', 1)) = LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    RightPointingDoubleAngleQuotationMark,  // Glyph 175 (('»', 1)) = RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    LightShade,                             // Glyph 176 (('░', 1)) = LIGHT SHADE
    MediumShade,                            // Glyph 177 (('▒', 1)) = MEDIUM SHADE
    DarkShade,                              // Glyph 178 (('▓', 1)) = DARK SHADE
    BoxDrawingsLightVertical,               // Glyph 179 (('│', 1)) = BOX DRAWINGS LIGHT VERTICAL
    BoxDrawingsLightVerticalAndLeft,        // Glyph 180 (('┤', 1)) = BOX DRAWINGS LIGHT VERTICAL AND LEFT
    LatinCapitalLetterAWithAcute,           // Glyph 181 (('Á', 1)) = LATIN CAPITAL LETTER A WITH ACUTE
    LatinCapitalLetterAWithCircumflex,      // Glyph 182 (('Â', 1)) = LATIN CAPITAL LETTER A WITH CIRCUMFLEX
    LatinCapitalLetterAWithGrave,           // Glyph 183 (('À', 1)) = LATIN CAPITAL LETTER A WITH GRAVE
    CopyrightSign,                          // Glyph 184 (('©', 1)) = COPYRIGHT SIGN
    BoxDrawingsDoubleVerticalAndLeft,       // Glyph 185 (('╣', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    BoxDrawingsDoubleVertical,              // Glyph 186 (('║', 1)) = BOX DRAWINGS DOUBLE VERTICAL
    BoxDrawingsDoubleDownAndLeft,           // Glyph 187 (('╗', 1)) = BOX DRAWINGS DOUBLE DOWN AND LEFT
    BoxDrawingsDoubleUpAndLeft,             // Glyph 188 (('╝', 1)) = BOX DRAWINGS DOUBLE UP AND LEFT
    CentSign,                               // Glyph 189 (('¢', 1)) = CENT SIGN
    YenSign,                                // Glyph 190 (('¥', 1)) = YEN SIGN
    BoxDrawingsLightDownAndLeft,            // Glyph 191 (('┐', 1)) = BOX DRAWINGS LIGHT DOWN AND LEFT
    BoxDrawingsLightUpAndRight,             // Glyph 192 (('└', 1)) = BOX DRAWINGS LIGHT UP AND RIGHT
    BoxDrawingsLightUpAndHorizontal,        // Glyph 193 (('┴', 1)) = BOX DRAWINGS LIGHT UP AND HORIZONTAL
    BoxDrawingsLightDownAndHorizontal,      // Glyph 194 (('┬', 1)) = BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    BoxDrawingsLightVerticalAndRight,       // Glyph 195 (('├', 1)) = BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    BoxDrawingsLightHorizontal,             // Glyph 196 (('─', 1)) = BOX DRAWINGS LIGHT HORIZONTAL
    BoxDrawingsLightVerticalAndHorizontal,  // Glyph 197 (('┼', 1)) = BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    LatinSmallLetterAWithTilde,             // Glyph 198 (('ã', 1)) = LATIN SMALL LETTER A WITH TILDE
    LatinCapitalLetterAWithTilde,           // Glyph 199 (('Ã', 1)) = LATIN CAPITAL LETTER A WITH TILDE
    BoxDrawingsDoubleUpAndRight,            // Glyph 200 (('╚', 1)) = BOX DRAWINGS DOUBLE UP AND RIGHT
    BoxDrawingsDoubleDownAndRight,          // Glyph 201 (('╔', 1)) = BOX DRAWINGS DOUBLE DOWN AND RIGHT
    BoxDrawingsDoubleUpAndHorizontal,       // Glyph 202 (('╩', 1)) = BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    BoxDrawingsDoubleDownAndHorizontal,     // Glyph 203 (('╦', 1)) = BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    BoxDrawingsDoubleVerticalAndRight,      // Glyph 204 (('╠', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    BoxDrawingsDoubleHorizontal,            // Glyph 205 (('═', 1)) = BOX DRAWINGS DOUBLE HORIZONTAL
    BoxDrawingsDoubleVerticalAndHorizontal, // Glyph 206 (('╬', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    CurrencySign,                           // Glyph 207 (('¤', 1)) = CURRENCY SIGN
    LatinSmallLetterEth,                    // Glyph 208 (('ð', 1)) = LATIN SMALL LETTER ETH
    LatinCapitalLetterEth,                  // Glyph 209 (('Ð', 1)) = LATIN CAPITAL LETTER ETH
    LatinCapitalLetterEWithCircumflex,      // Glyph 210 (('Ê', 1)) = LATIN CAPITAL LETTER E WITH CIRCUMFLEX
    LatinCapitalLetterEWithDiaeresis,       // Glyph 211 (('Ë', 1)) = LATIN CAPITAL LETTER E WITH DIAERESIS
    LatinCapitalLetterEWithGrave,           // Glyph 212 (('È', 1)) = LATIN CAPITAL LETTER E WITH GRAVE
    LatinSmallLetterDotlessI,               // Glyph 213 (('ı', 1)) = LATIN SMALL LETTER DOTLESS I
    LatinCapitalLetterIWithAcute,           // Glyph 214 (('Í', 1)) = LATIN CAPITAL LETTER I WITH ACUTE
    LatinCapitalLetterIWithCircumflex,      // Glyph 215 (('Î', 1)) = LATIN CAPITAL LETTER I WITH CIRCUMFLEX
    LatinCapitalLetterIWithDiaeresis,       // Glyph 216 (('Ï', 1)) = LATIN CAPITAL LETTER I WITH DIAERESIS
    BoxDrawingsLightUpAndLeft,              // Glyph 217 (('┘', 1)) = BOX DRAWINGS LIGHT UP AND LEFT
    BoxDrawingsLightDownAndRight,           // Glyph 218 (('┌', 1)) = BOX DRAWINGS LIGHT DOWN AND RIGHT
    FullBlock,                              // Glyph 219 (('█', 1)) = FULL BLOCK
    LowerHalfBlock,                         // Glyph 220 (('▄', 1)) = LOWER HALF BLOCK
    BrokenBar,                              // Glyph 221 (('¦', 1)) = BROKEN BAR
    LatinCapitalLetterIWithGrave,           // Glyph 222 (('Ì', 1)) = LATIN CAPITAL LETTER I WITH GRAVE
    UpperHalfBlock,                         // Glyph 223 (('▀', 1)) = UPPER HALF BLOCK
    LatinCapitalLetterOWithAcute,           // Glyph 224 (('Ó', 1)) = LATIN CAPITAL LETTER O WITH ACUTE
    LatinSmallLetterSharpS,                 // Glyph 225 (('ß', 1)) = LATIN SMALL LETTER SHARP S
    LatinCapitalLetterOWithCircumflex,      // Glyph 226 (('Ô', 1)) = LATIN CAPITAL LETTER O WITH CIRCUMFLEX
    LatinCapitalLetterOWithGrave,           // Glyph 227 (('Ò', 1)) = LATIN CAPITAL LETTER O WITH GRAVE
    LatinSmallLetterOWithTilde,             // Glyph 228 (('õ', 1)) = LATIN SMALL LETTER O WITH TILDE
    LatinCapitalLetterOWithTilde,           // Glyph 229 (('Õ', 1)) = LATIN CAPITAL LETTER O WITH TILDE
    MicroSign,                              // Glyph 230 (('µ', 1)) = MICRO SIGN
    LatinSmallLetterThorn,                  // Glyph 231 (('þ', 1)) = LATIN SMALL LETTER THORN
    LatinCapitalLetterThorn,                // Glyph 232 (('Þ', 1)) = LATIN CAPITAL LETTER THORN
    LatinCapitalLetterUWithAcute,           // Glyph 233 (('Ú', 1)) = LATIN CAPITAL LETTER U WITH ACUTE
    LatinCapitalLetterUWithCircumflex,      // Glyph 234 (('Û', 1)) = LATIN CAPITAL LETTER U WITH CIRCUMFLEX
    LatinCapitalLetterUWithGrave,           // Glyph 235 (('Ù', 1)) = LATIN CAPITAL LETTER U WITH GRAVE
    LatinSmallLetterYWithAcute,             // Glyph 236 (('ý', 1)) = LATIN SMALL LETTER Y WITH ACUTE
    LatinCapitalLetterYWithAcute,           // Glyph 237 (('Ý', 1)) = LATIN CAPITAL LETTER Y WITH ACUTE
    Macron,                                 // Glyph 238 (('¯', 1)) = MACRON
    AcuteAccent,                            // Glyph 239 (('´', 1)) = ACUTE ACCENT
    SoftHyphen,                             // Glyph 240 (('­­­­­­-', 1)) = SOFT HYPHEN
    PlusMinusSign,                          // Glyph 241 (('±', 1)) = PLUS-MINUS SIGN
    DoubleLowLine,                          // Glyph 242 (('‗', 1)) = DOUBLE LOW LINE
    VulgarFractionThreeQuarters,            // Glyph 243 (('¾', 1)) = VULGAR FRACTION THREE QUARTERS
    PilcrowSign,                            // Glyph 244 (('¶', 1)) = PILCROW SIGN
    SectionSign,                            // Glyph 245 (('§', 1)) = SECTION SIGN
    DivisionSign,                           // Glyph 246 (('÷', 1)) = DIVISION SIGN
    Cedilla,                                // Glyph 247 (('¸', 1)) = CEDILLA
    DegreeSign,                             // Glyph 248 (('°', 1)) = DEGREE SIGN
    Diaeresis,                              // Glyph 249 (('¨', 1)) = DIAERESIS
    MiddleDot,                              // Glyph 250 (('·', 1)) = MIDDLE DOT
    SuperscriptOne,                         // Glyph 251 (('¹', 1)) = SUPERSCRIPT ONE
    SuperscriptThree,                       // Glyph 252 (('³', 1)) = SUPERSCRIPT THREE
    SuperscriptTwo,                         // Glyph 253 (('²', 1)) = SUPERSCRIPT TWO
    BlackSquare,                            // Glyph 254 (('■', 1)) = BLACK SQUARE
    NoBreakSpace,                           // Glyph 255 (('\xa0', 1)) = NO-BREAK SPACE
}

impl ::core::default::Default for Glyph {
    fn default() -> Glyph {
        Glyph::Space
    }
}

/// The font data here must be in the same order as the `Glyph` enum. This is
/// the cp850-8x16 font from FreeBSD. See
/// http://web.mit.edu/freebsd/head/share/syscons/fonts/cp850-8x16.fnt
///
/// The compilation of software known as FreeBSD is distributed under the
/// following terms:
///
/// Copyright (c) 1992-2014 The FreeBSD Project. All rights reserved.
///
/// Redistribution and use in source and binary forms, with or without
/// modification, are permitted provided that the following conditions
/// are met:
/// 1. Redistributions of source code must retain the above copyright
///    notice, this list of conditions and the following disclaimer.
/// 2. Redistributions in binary form must reproduce the above copyright
///    notice, this list of conditions and the following disclaimer in the
///    documentation and/or other materials provided with the distribution.
///
/// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
/// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
/// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
/// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
/// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
/// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
/// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
/// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
/// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
/// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
/// SUCH DAMAGE.
pub const FONT_DATA: [u8; 256 * 16] = [
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Null
    0x00,0x00,0x7e,0x81,0xa5,0x81,0x81,0xbd,0x99,0x81,0x81,0x7e,0x00,0x00,0x00,0x00, // Glyph::SOH
    0x00,0x00,0x7e,0xff,0xdb,0xff,0xff,0xc3,0xe7,0xff,0xff,0x7e,0x00,0x00,0x00,0x00, // Glyph::STX
    0x00,0x00,0x00,0x00,0x6c,0xfe,0xfe,0xfe,0xfe,0x7c,0x38,0x10,0x00,0x00,0x00,0x00, // Glyph::ETX
    0x00,0x00,0x00,0x00,0x10,0x38,0x7c,0xfe,0x7c,0x38,0x10,0x00,0x00,0x00,0x00,0x00, // Glyph::EOT
    0x00,0x00,0x00,0x18,0x3c,0x3c,0xe7,0xe7,0xe7,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::ENQ
    0x00,0x00,0x00,0x18,0x3c,0x7e,0xff,0xff,0x7e,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::ACK
    0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x3c,0x3c,0x18,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BEL
    0xff,0xff,0xff,0xff,0xff,0xff,0xe7,0xc3,0xc3,0xe7,0xff,0xff,0xff,0xff,0xff,0xff, // Glyph::BS
    0x00,0x00,0x00,0x00,0x00,0x3c,0x66,0x42,0x42,0x66,0x3c,0x00,0x00,0x00,0x00,0x00, // Glyph::HT
    0xff,0xff,0xff,0xff,0xff,0xc3,0x99,0xbd,0xbd,0x99,0xc3,0xff,0xff,0xff,0xff,0xff, // Glyph::LF
    0x00,0x00,0x1e,0x0e,0x1a,0x32,0x78,0xcc,0xcc,0xcc,0xcc,0x78,0x00,0x00,0x00,0x00, // Glyph::VT
    0x00,0x00,0x3c,0x66,0x66,0x66,0x66,0x3c,0x18,0x7e,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::FF
    0x00,0x00,0x3f,0x33,0x3f,0x30,0x30,0x30,0x30,0x70,0xf0,0xe0,0x00,0x00,0x00,0x00, // Glyph::CR
    0x00,0x00,0x7f,0x63,0x7f,0x63,0x63,0x63,0x63,0x67,0xe7,0xe6,0xc0,0x00,0x00,0x00, // Glyph::SO
    0x00,0x00,0x00,0x18,0x18,0xdb,0x3c,0xe7,0x3c,0xdb,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::SI
    0x00,0x80,0xc0,0xe0,0xf0,0xf8,0xfe,0xf8,0xf0,0xe0,0xc0,0x80,0x00,0x00,0x00,0x00, // Glyph::DLE
    0x00,0x02,0x06,0x0e,0x1e,0x3e,0xfe,0x3e,0x1e,0x0e,0x06,0x02,0x00,0x00,0x00,0x00, // Glyph::DC1
    0x00,0x00,0x18,0x3c,0x7e,0x18,0x18,0x18,0x7e,0x3c,0x18,0x00,0x00,0x00,0x00,0x00, // Glyph::DC2
    0x00,0x00,0x66,0x66,0x66,0x66,0x66,0x66,0x66,0x00,0x66,0x66,0x00,0x00,0x00,0x00, // Glyph::DC3
    0x00,0x00,0x7f,0xdb,0xdb,0xdb,0x7b,0x1b,0x1b,0x1b,0x1b,0x1b,0x00,0x00,0x00,0x00, // Glyph::DC4
    0x00,0x7c,0xc6,0x60,0x38,0x6c,0xc6,0xc6,0x6c,0x38,0x0c,0xc6,0x7c,0x00,0x00,0x00, // Glyph::NAK
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xfe,0xfe,0xfe,0xfe,0x00,0x00,0x00,0x00, // Glyph::SYN
    0x00,0x00,0x18,0x3c,0x7e,0x18,0x18,0x18,0x7e,0x3c,0x18,0x7e,0x00,0x00,0x00,0x00, // Glyph::ETB
    0x00,0x00,0x18,0x3c,0x7e,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::CAN
    0x00,0x00,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x7e,0x3c,0x18,0x00,0x00,0x00,0x00, // Glyph::EM
    0x00,0x00,0x00,0x00,0x00,0x18,0x0c,0xfe,0x0c,0x18,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::SUB
    0x00,0x00,0x00,0x00,0x00,0x30,0x60,0xfe,0x60,0x30,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Escape
    0x00,0x00,0x00,0x00,0x00,0x00,0xc0,0xc0,0xc0,0xfe,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::FS
    0x00,0x00,0x00,0x00,0x00,0x28,0x6c,0xfe,0x6c,0x28,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::GS
    0x00,0x00,0x00,0x00,0x10,0x38,0x38,0x7c,0x7c,0xfe,0xfe,0x00,0x00,0x00,0x00,0x00, // Glyph::RS
    0x00,0x00,0x00,0x00,0xfe,0xfe,0x7c,0x7c,0x38,0x38,0x10,0x00,0x00,0x00,0x00,0x00, // Glyph::US
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Space
    0x00,0x00,0x18,0x3c,0x3c,0x3c,0x18,0x18,0x18,0x00,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::ExclamationMark
    0x00,0x66,0x66,0x66,0x24,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::QuotationMark
    0x00,0x00,0x00,0x6c,0x6c,0xfe,0x6c,0x6c,0x6c,0xfe,0x6c,0x6c,0x00,0x00,0x00,0x00, // Glyph::NumberSign
    0x18,0x18,0x7c,0xc6,0xc2,0xc0,0x7c,0x06,0x06,0x86,0xc6,0x7c,0x18,0x18,0x00,0x00, // Glyph::DollarSign
    0x00,0x00,0x00,0x00,0xc2,0xc6,0x0c,0x18,0x30,0x60,0xc6,0x86,0x00,0x00,0x00,0x00, // Glyph::PercentSign
    0x00,0x00,0x38,0x6c,0x6c,0x38,0x76,0xdc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::Ampersand
    0x00,0x30,0x30,0x30,0x60,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Apostrophe
    0x00,0x00,0x0c,0x18,0x30,0x30,0x30,0x30,0x30,0x30,0x18,0x0c,0x00,0x00,0x00,0x00, // Glyph::LeftParenthesis
    0x00,0x00,0x30,0x18,0x0c,0x0c,0x0c,0x0c,0x0c,0x0c,0x18,0x30,0x00,0x00,0x00,0x00, // Glyph::RightParenthesis
    0x00,0x00,0x00,0x00,0x00,0x66,0x3c,0xff,0x3c,0x66,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Asterisk
    0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x7e,0x18,0x18,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::PlusSign
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x18,0x30,0x00,0x00,0x00, // Glyph::Comma
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::HyphenMinus
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::FullStop
    0x00,0x00,0x00,0x00,0x02,0x06,0x0c,0x18,0x30,0x60,0xc0,0x80,0x00,0x00,0x00,0x00, // Glyph::Solidus
    0x00,0x00,0x38,0x6c,0xc6,0xc6,0xd6,0xd6,0xc6,0xc6,0x6c,0x38,0x00,0x00,0x00,0x00, // Glyph::DigitZero
    0x00,0x00,0x18,0x38,0x78,0x18,0x18,0x18,0x18,0x18,0x18,0x7e,0x00,0x00,0x00,0x00, // Glyph::DigitOne
    0x00,0x00,0x7c,0xc6,0x06,0x0c,0x18,0x30,0x60,0xc0,0xc6,0xfe,0x00,0x00,0x00,0x00, // Glyph::DigitTwo
    0x00,0x00,0x7c,0xc6,0x06,0x06,0x3c,0x06,0x06,0x06,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::DigitThree
    0x00,0x00,0x0c,0x1c,0x3c,0x6c,0xcc,0xfe,0x0c,0x0c,0x0c,0x1e,0x00,0x00,0x00,0x00, // Glyph::DigitFour
    0x00,0x00,0xfe,0xc0,0xc0,0xc0,0xfc,0x06,0x06,0x06,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::DigitFive
    0x00,0x00,0x38,0x60,0xc0,0xc0,0xfc,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::DigitSix
    0x00,0x00,0xfe,0xc6,0x06,0x06,0x0c,0x18,0x30,0x30,0x30,0x30,0x00,0x00,0x00,0x00, // Glyph::DigitSeven
    0x00,0x00,0x7c,0xc6,0xc6,0xc6,0x7c,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::DigitEight
    0x00,0x00,0x7c,0xc6,0xc6,0xc6,0x7e,0x06,0x06,0x06,0x0c,0x78,0x00,0x00,0x00,0x00, // Glyph::DigitNine
    0x00,0x00,0x00,0x00,0x18,0x18,0x00,0x00,0x00,0x18,0x18,0x00,0x00,0x00,0x00,0x00, // Glyph::Colon
    0x00,0x00,0x00,0x00,0x18,0x18,0x00,0x00,0x00,0x18,0x18,0x30,0x00,0x00,0x00,0x00, // Glyph::Semicolon
    0x00,0x00,0x00,0x06,0x0c,0x18,0x30,0x60,0x30,0x18,0x0c,0x06,0x00,0x00,0x00,0x00, // Glyph::LessThanSign
    0x00,0x00,0x00,0x00,0x00,0x7e,0x00,0x00,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::EqualsSign
    0x00,0x00,0x00,0x60,0x30,0x18,0x0c,0x06,0x0c,0x18,0x30,0x60,0x00,0x00,0x00,0x00, // Glyph::GreaterThanSign
    0x00,0x00,0x7c,0xc6,0xc6,0x0c,0x18,0x18,0x18,0x00,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::QuestionMark
    0x00,0x00,0x00,0x7c,0xc6,0xc6,0xde,0xde,0xde,0xdc,0xc0,0x7c,0x00,0x00,0x00,0x00, // Glyph::CommercialAt
    0x00,0x00,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterA
    0x00,0x00,0xfc,0x66,0x66,0x66,0x7c,0x66,0x66,0x66,0x66,0xfc,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterB
    0x00,0x00,0x3c,0x66,0xc2,0xc0,0xc0,0xc0,0xc0,0xc2,0x66,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterC
    0x00,0x00,0xf8,0x6c,0x66,0x66,0x66,0x66,0x66,0x66,0x6c,0xf8,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterD
    0x00,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x60,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterE
    0x00,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x60,0x60,0x60,0xf0,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterF
    0x00,0x00,0x3c,0x66,0xc2,0xc0,0xc0,0xde,0xc6,0xc6,0x66,0x3a,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterG
    0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterH
    0x00,0x00,0x3c,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterI
    0x00,0x00,0x1e,0x0c,0x0c,0x0c,0x0c,0x0c,0xcc,0xcc,0xcc,0x78,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterJ
    0x00,0x00,0xe6,0x66,0x66,0x6c,0x78,0x78,0x6c,0x66,0x66,0xe6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterK
    0x00,0x00,0xf0,0x60,0x60,0x60,0x60,0x60,0x60,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterL
    0x00,0x00,0xc6,0xee,0xfe,0xfe,0xd6,0xc6,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterM
    0x00,0x00,0xc6,0xe6,0xf6,0xfe,0xde,0xce,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterN
    0x00,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterO
    0x00,0x00,0xfc,0x66,0x66,0x66,0x7c,0x60,0x60,0x60,0x60,0xf0,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterP
    0x00,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xd6,0xde,0x7c,0x0c,0x0e,0x00,0x00, // Glyph::LatinCapitalLetterQ
    0x00,0x00,0xfc,0x66,0x66,0x66,0x7c,0x6c,0x66,0x66,0x66,0xe6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterR
    0x00,0x00,0x7c,0xc6,0xc6,0x60,0x38,0x0c,0x06,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterS
    0x00,0x00,0x7e,0x7e,0x5a,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterT
    0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterU
    0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x6c,0x38,0x10,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterV
    0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xd6,0xd6,0xd6,0xfe,0xee,0x6c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterW
    0x00,0x00,0xc6,0xc6,0x6c,0x7c,0x38,0x38,0x7c,0x6c,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterX
    0x00,0x00,0x66,0x66,0x66,0x66,0x3c,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterY
    0x00,0x00,0xfe,0xc6,0x86,0x0c,0x18,0x30,0x60,0xc2,0xc6,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterZ
    0x00,0x00,0x3c,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x3c,0x00,0x00,0x00,0x00, // Glyph::LeftSquareBracket
    0x00,0x00,0x00,0x80,0xc0,0xe0,0x70,0x38,0x1c,0x0e,0x06,0x02,0x00,0x00,0x00,0x00, // Glyph::ReverseSolidus
    0x00,0x00,0x3c,0x0c,0x0c,0x0c,0x0c,0x0c,0x0c,0x0c,0x0c,0x3c,0x00,0x00,0x00,0x00, // Glyph::RightSquareBracket
    0x10,0x38,0x6c,0xc6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::CircumflexAccent
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0x00,0x00, // Glyph::LowLine
    0x00,0x30,0x18,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::GraveAccent
    0x00,0x00,0x00,0x00,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterA
    0x00,0x00,0xe0,0x60,0x60,0x78,0x6c,0x66,0x66,0x66,0x66,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterB
    0x00,0x00,0x00,0x00,0x00,0x7c,0xc6,0xc0,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterC
    0x00,0x00,0x1c,0x0c,0x0c,0x3c,0x6c,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterD
    0x00,0x00,0x00,0x00,0x00,0x7c,0xc6,0xfe,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterE
    0x00,0x00,0x1c,0x36,0x32,0x30,0x78,0x30,0x30,0x30,0x30,0x78,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterF
    0x00,0x00,0x00,0x00,0x00,0x76,0xcc,0xcc,0xcc,0xcc,0xcc,0x7c,0x0c,0xcc,0x78,0x00, // Glyph::LatinSmallLetterG
    0x00,0x00,0xe0,0x60,0x60,0x6c,0x76,0x66,0x66,0x66,0x66,0xe6,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterH
    0x00,0x00,0x18,0x18,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterI
    0x00,0x00,0x06,0x06,0x00,0x0e,0x06,0x06,0x06,0x06,0x06,0x06,0x66,0x66,0x3c,0x00, // Glyph::LatinSmallLetterJ
    0x00,0x00,0xe0,0x60,0x60,0x66,0x6c,0x78,0x78,0x6c,0x66,0xe6,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterK
    0x00,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterL
    0x00,0x00,0x00,0x00,0x00,0xec,0xfe,0xd6,0xd6,0xd6,0xd6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterM
    0x00,0x00,0x00,0x00,0x00,0xdc,0x66,0x66,0x66,0x66,0x66,0x66,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterN
    0x00,0x00,0x00,0x00,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterO
    0x00,0x00,0x00,0x00,0x00,0xdc,0x66,0x66,0x66,0x66,0x66,0x7c,0x60,0x60,0xf0,0x00, // Glyph::LatinSmallLetterP
    0x00,0x00,0x00,0x00,0x00,0x76,0xcc,0xcc,0xcc,0xcc,0xcc,0x7c,0x0c,0x0c,0x1e,0x00, // Glyph::LatinSmallLetterQ
    0x00,0x00,0x00,0x00,0x00,0xdc,0x76,0x66,0x60,0x60,0x60,0xf0,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterR
    0x00,0x00,0x00,0x00,0x00,0x7c,0xc6,0x60,0x38,0x0c,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterS
    0x00,0x00,0x10,0x30,0x30,0xfc,0x30,0x30,0x30,0x30,0x36,0x1c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterT
    0x00,0x00,0x00,0x00,0x00,0xcc,0xcc,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterU
    0x00,0x00,0x00,0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0x6c,0x38,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterV
    0x00,0x00,0x00,0x00,0x00,0xc6,0xc6,0xd6,0xd6,0xd6,0xfe,0x6c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterW
    0x00,0x00,0x00,0x00,0x00,0xc6,0x6c,0x38,0x38,0x38,0x6c,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterX
    0x00,0x00,0x00,0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7e,0x06,0x0c,0xf8,0x00, // Glyph::LatinSmallLetterY
    0x00,0x00,0x00,0x00,0x00,0xfe,0xcc,0x18,0x30,0x60,0xc6,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterZ
    0x00,0x00,0x0e,0x18,0x18,0x18,0x70,0x18,0x18,0x18,0x18,0x0e,0x00,0x00,0x00,0x00, // Glyph::LeftCurlyBracket
    0x00,0x00,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::VerticalLine
    0x00,0x00,0x70,0x18,0x18,0x18,0x0e,0x18,0x18,0x18,0x18,0x70,0x00,0x00,0x00,0x00, // Glyph::RightCurlyBracket
    0x00,0x76,0xdc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Tilde
    0x00,0x00,0x00,0x00,0x10,0x38,0x6c,0xc6,0xc6,0xc6,0xfe,0x00,0x00,0x00,0x00,0x00, // Glyph::Delete
    0x00,0x00,0x3c,0x66,0xc2,0xc0,0xc0,0xc0,0xc0,0xc2,0x66,0x3c,0x18,0x70,0x00,0x00, // Glyph::LatinCapitalLetterCWithCedilla
    0x00,0x00,0xcc,0x00,0x00,0xcc,0xcc,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterUWithDiaeresis
    0x00,0x0c,0x18,0x30,0x00,0x7c,0xc6,0xfe,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterEWithAcute
    0x00,0x10,0x38,0x6c,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithCircumflex
    0x00,0x00,0xcc,0x00,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithDiaeresis
    0x00,0x60,0x30,0x18,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithGrave
    0x00,0x38,0x6c,0x38,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithRingAbove
    0x00,0x00,0x00,0x00,0x00,0x7c,0xc6,0xc0,0xc0,0xc0,0xc6,0x7c,0x18,0x70,0x00,0x00, // Glyph::LatinSmallLetterCWithCedilla
    0x00,0x10,0x38,0x6c,0x00,0x7c,0xc6,0xfe,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterEWithCircumflex
    0x00,0x00,0xc6,0x00,0x00,0x7c,0xc6,0xfe,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterEWithDiaeresis
    0x00,0x60,0x30,0x18,0x00,0x7c,0xc6,0xfe,0xc0,0xc0,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterEWithGrave
    0x00,0x00,0x66,0x00,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterIWithDiaeresis
    0x00,0x18,0x3c,0x66,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterIWithCircumflex
    0x00,0x60,0x30,0x18,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterIWithGrave
    0x00,0xc6,0x00,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithDiaeresis
    0x38,0x6c,0x38,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithRingAbove
    0x0c,0x18,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterEWithAcute
    0x00,0x00,0x00,0x00,0x00,0xec,0x36,0x36,0x7e,0xd8,0xd8,0x6e,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAe
    0x00,0x00,0x3e,0x6c,0xcc,0xcc,0xfe,0xcc,0xcc,0xcc,0xcc,0xce,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAe
    0x00,0x10,0x38,0x6c,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithCircumflex
    0x00,0x00,0xc6,0x00,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithDiaeresis
    0x00,0x60,0x30,0x18,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithGrave
    0x00,0x30,0x78,0xcc,0x00,0xcc,0xcc,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterUWithCircumflex
    0x00,0x60,0x30,0x18,0x00,0xcc,0xcc,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterUWithGrave
    0x00,0x00,0xc6,0x00,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7e,0x06,0x0c,0x78,0x00, // Glyph::LatinSmallLetterYWithDiaeresis
    0x00,0xc6,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithDiaeresis
    0x00,0xc6,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterUWithDiaeresis
    0x00,0x00,0x00,0x00,0x00,0x7c,0xce,0xde,0xf6,0xe6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithStroke
    0x00,0x38,0x6c,0x64,0x60,0xf0,0x60,0x60,0x60,0x60,0xe6,0xfc,0x00,0x00,0x00,0x00, // Glyph::PoundSign
    0x00,0x04,0x7c,0xce,0xce,0xd6,0xd6,0xd6,0xd6,0xe6,0xe6,0x7c,0x40,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithStroke
    0x00,0x00,0x00,0x00,0x00,0xc6,0x6c,0x38,0x38,0x6c,0xc6,0x00,0x00,0x00,0x00,0x00, // Glyph::MultiplicationSign
    0x00,0x0e,0x1b,0x18,0x18,0x18,0x7e,0x18,0x18,0x18,0xd8,0x70,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterFWithHook
    0x00,0x18,0x30,0x60,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithAcute
    0x00,0x0c,0x18,0x30,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterIWithAcute
    0x00,0x18,0x30,0x60,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithAcute
    0x00,0x18,0x30,0x60,0x00,0xcc,0xcc,0xcc,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterUWithAcute
    0x00,0x00,0x76,0xdc,0x00,0xdc,0x66,0x66,0x66,0x66,0x66,0x66,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterNWithTilde
    0x76,0xdc,0x00,0xc6,0xe6,0xf6,0xfe,0xde,0xce,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterNWithTilde
    0x00,0x00,0x3c,0x6c,0x6c,0x3e,0x00,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::FeminineOrdinalIndicator
    0x00,0x00,0x38,0x6c,0x6c,0x38,0x00,0x7c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::MasculineOrdinalIndicator
    0x00,0x00,0x30,0x30,0x00,0x30,0x30,0x60,0xc0,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::InvertedQuestionMark
    0x00,0x00,0x7c,0x82,0xb2,0xaa,0xb2,0xaa,0xaa,0x82,0x7c,0x00,0x00,0x00,0x00,0x00, // Glyph::RegisteredSign
    0x00,0x00,0x00,0x00,0x00,0x00,0xfe,0x06,0x06,0x06,0x06,0x00,0x00,0x00,0x00,0x00, // Glyph::NotSign
    0x00,0x60,0xe0,0x62,0x66,0x6c,0x18,0x30,0x60,0xdc,0x86,0x0c,0x18,0x3e,0x00,0x00, // Glyph::VulgarFractionOneHalf
    0x00,0x60,0xe0,0x62,0x66,0x6c,0x18,0x30,0x66,0xce,0x9a,0x3f,0x06,0x06,0x00,0x00, // Glyph::VulgarFractionOneQuarter
    0x00,0x00,0x18,0x18,0x00,0x18,0x18,0x18,0x3c,0x3c,0x3c,0x18,0x00,0x00,0x00,0x00, // Glyph::InvertedExclamationMark
    0x00,0x00,0x00,0x00,0x00,0x36,0x6c,0xd8,0x6c,0x36,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::LeftPointingDoubleAngleQuotationMark
    0x00,0x00,0x00,0x00,0x00,0xd8,0x6c,0x36,0x6c,0xd8,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::RightPointingDoubleAngleQuotationMark
    0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44,0x11,0x44, // Glyph::LightShade
    0x55,0xaa,0x55,0xaa,0x55,0xaa,0x55,0xaa,0x55,0xaa,0x55,0xaa,0x55,0xaa,0x55,0xaa, // Glyph::MediumShade
    0xdd,0x77,0xdd,0x77,0xdd,0x77,0xdd,0x77,0xdd,0x77,0xdd,0x77,0xdd,0x77,0xdd,0x77, // Glyph::DarkShade
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightVertical
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0xf8,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightVerticalAndLeft
    0x60,0xc0,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithAcute
    0x7c,0xc6,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithCircumflex
    0x0c,0x06,0x10,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithGrave
    0x00,0x00,0x7c,0x82,0x9a,0xa2,0xa2,0xa2,0x9a,0x82,0x7c,0x00,0x00,0x00,0x00,0x00, // Glyph::CopyrightSign
    0x36,0x36,0x36,0x36,0x36,0xf6,0x06,0xf6,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleVerticalAndLeft
    0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleVertical
    0x00,0x00,0x00,0x00,0x00,0xfe,0x06,0xf6,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleDownAndLeft
    0x36,0x36,0x36,0x36,0x36,0xf6,0x06,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsDoubleUpAndLeft
    0x00,0x00,0x18,0x18,0x7c,0xc6,0xc0,0xc0,0xc6,0x7c,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::CentSign
    0x00,0x00,0x00,0x66,0x66,0x3c,0x18,0x7e,0x18,0x7e,0x18,0x18,0x00,0x00,0x00,0x00, // Glyph::YenSign
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xf8,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightDownAndLeft
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x1f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsLightUpAndRight
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsLightUpAndHorizontal
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightDownAndHorizontal
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x1f,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightVerticalAndRight
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsLightHorizontal
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0xff,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightVerticalAndHorizontal
    0x00,0x00,0x76,0xdc,0x00,0x78,0x0c,0x7c,0xcc,0xcc,0xcc,0x76,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterAWithTilde
    0x76,0xdc,0x00,0x38,0x6c,0xc6,0xc6,0xfe,0xc6,0xc6,0xc6,0xc6,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterAWithTilde
    0x36,0x36,0x36,0x36,0x36,0x37,0x30,0x3f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsDoubleUpAndRight
    0x00,0x00,0x00,0x00,0x00,0x3f,0x30,0x37,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleDownAndRight
    0x36,0x36,0x36,0x36,0x36,0xf7,0x00,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsDoubleUpAndHorizontal
    0x00,0x00,0x00,0x00,0x00,0xff,0x00,0xf7,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleDownAndHorizontal
    0x36,0x36,0x36,0x36,0x36,0x37,0x30,0x37,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleVerticalAndRight
    0x00,0x00,0x00,0x00,0x00,0xff,0x00,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsDoubleHorizontal
    0x36,0x36,0x36,0x36,0x36,0xf7,0x00,0xf7,0x36,0x36,0x36,0x36,0x36,0x36,0x36,0x36, // Glyph::BoxDrawingsDoubleVerticalAndHorizontal
    0x00,0x00,0x00,0x00,0xc6,0x7c,0xc6,0xc6,0xc6,0xc6,0x7c,0xc6,0x00,0x00,0x00,0x00, // Glyph::CurrencySign
    0x00,0x00,0x34,0x18,0x2c,0x06,0x3e,0x66,0x66,0x66,0x66,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterEth
    0x00,0x00,0xf8,0x6c,0x66,0x66,0xf6,0x66,0x66,0x66,0x6c,0xf8,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterEth
    0x38,0x6c,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterEWithCircumflex
    0x00,0xc6,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterEWithDiaeresis
    0x30,0x18,0x00,0xfe,0x66,0x62,0x68,0x78,0x68,0x62,0x66,0xfe,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterEWithGrave
    0x00,0x00,0x00,0x00,0x00,0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterDotlessI
    0x0c,0x18,0x00,0x3c,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterIWithAcute
    0x3c,0x66,0x00,0x3c,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterIWithCircumflex
    0x00,0x66,0x00,0x3c,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterIWithDiaeresis
    0x18,0x18,0x18,0x18,0x18,0x18,0x18,0xf8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::BoxDrawingsLightUpAndLeft
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x1f,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18, // Glyph::BoxDrawingsLightDownAndRight
    0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff, // Glyph::FullBlock
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff, // Glyph::LowerHalfBlock
    0x00,0x18,0x18,0x18,0x18,0x18,0x00,0x00,0x18,0x18,0x18,0x18,0x18,0x00,0x00,0x00, // Glyph::BrokenBar
    0x30,0x18,0x00,0x3c,0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterIWithGrave
    0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::UpperHalfBlock
    0x18,0x30,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithAcute
    0x00,0x00,0x78,0xcc,0xcc,0xcc,0xd8,0xcc,0xc6,0xc6,0xc6,0xcc,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterSharpS
    0x38,0x6c,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithCircumflex
    0x30,0x18,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithGrave
    0x00,0x00,0x76,0xdc,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinSmallLetterOWithTilde
    0x76,0xdc,0x00,0x7c,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterOWithTilde
    0x00,0x00,0x00,0x00,0x00,0x66,0x66,0x66,0x66,0x66,0x66,0x7c,0x60,0x60,0xc0,0x00, // Glyph::MicroSign
    0x00,0x00,0xe0,0x60,0x60,0x7c,0x66,0x66,0x66,0x66,0x66,0x7c,0x60,0x60,0xf0,0x00, // Glyph::LatinSmallLetterThorn
    0x00,0x00,0xf0,0x60,0x7c,0x66,0x66,0x66,0x66,0x7c,0x60,0xf0,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterThorn
    0x18,0x30,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterUWithAcute
    0x38,0x6c,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterUWithCircumflex
    0x30,0x18,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterUWithGrave
    0x00,0x0c,0x18,0x30,0x00,0xc6,0xc6,0xc6,0xc6,0xc6,0xc6,0x7e,0x06,0x0c,0xf8,0x00, // Glyph::LatinSmallLetterYWithAcute
    0x0c,0x18,0x00,0x66,0x66,0x66,0x66,0x3c,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00, // Glyph::LatinCapitalLetterYWithAcute
    0x00,0xff,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Macron
    0x00,0x0c,0x18,0x30,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::AcuteAccent
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::SoftHyphen
    0x00,0x00,0x00,0x00,0x18,0x18,0x7e,0x18,0x18,0x00,0x00,0x7e,0x00,0x00,0x00,0x00, // Glyph::PlusMinusSign
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0x00,0xff,0x00, // Glyph::DoubleLowLine
    0x00,0xe0,0x30,0x62,0x36,0xec,0x18,0x30,0x66,0xce,0x9a,0x3f,0x06,0x06,0x00,0x00, // Glyph::VulgarFractionThreeQuarters
    0x00,0x00,0x7f,0xdb,0xdb,0xdb,0x7b,0x1b,0x1b,0x1b,0x1b,0x1b,0x00,0x00,0x00,0x00, // Glyph::PilcrowSign
    0x00,0x7c,0xc6,0x60,0x38,0x6c,0xc6,0xc6,0x6c,0x38,0x0c,0xc6,0x7c,0x00,0x00,0x00, // Glyph::SectionSign
    0x00,0x00,0x00,0x00,0x00,0x18,0x00,0x7e,0x00,0x18,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::DivisionSign
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x0c,0x78,0x00,0x00, // Glyph::Cedilla
    0x00,0x38,0x6c,0x6c,0x38,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::DegreeSign
    0x00,0xc6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::Diaeresis
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x18,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::MiddleDot
    0x00,0x18,0x38,0x18,0x18,0x18,0x3c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::SuperscriptOne
    0x00,0x7c,0x06,0x3c,0x06,0x06,0x7c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::SuperscriptThree
    0x00,0x3c,0x66,0x0c,0x18,0x32,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::SuperscriptTwo
    0x00,0x00,0x00,0x00,0x7e,0x7e,0x7e,0x7e,0x7e,0x7e,0x7e,0x00,0x00,0x00,0x00,0x00, // Glyph::BlackSquare
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // Glyph::NoBreakSpace
];

impl Glyph {
    pub fn map_char(ch: char) -> Glyph {
        match ch {
            // Add more CP850 mappings here - hearts, clubs, diamonds, etc
            '\u{0000}' => Glyph::Null,   // Glyph 0 (('\x00', 1)) = UNKNOWN
            '\u{0001}' => Glyph::SOH,    // Glyph 1 (('\x01', 1)) = UNKNOWN
            '\u{0002}' => Glyph::STX,    // Glyph 2 (('\x02', 1)) = UNKNOWN
            '\u{0003}' => Glyph::ETX,    // Glyph 3 (('\x03', 1)) = UNKNOWN
            '\u{0004}' => Glyph::EOT,    // Glyph 4 (('\x04', 1)) = UNKNOWN
            '\u{0005}' => Glyph::ENQ,    // Glyph 5 (('\x05', 1)) = UNKNOWN
            '\u{0006}' => Glyph::ACK,    // Glyph 6 (('\x06', 1)) = UNKNOWN
            '\u{0007}' => Glyph::BEL,    // Glyph 7 (('\x07', 1)) = UNKNOWN
            '\u{0008}' => Glyph::BS,     // Glyph 8 (('\x08', 1)) = UNKNOWN
            '\t' => Glyph::HT,           // Glyph 9 (('\t', 1)) = UNKNOWN
            '\n' => Glyph::LF,           // Glyph 10 (('\n', 1)) = UNKNOWN
            '\u{000b}' => Glyph::VT,     // Glyph 11 (('\x0b', 1)) = UNKNOWN
            '\u{000c}' => Glyph::FF,     // Glyph 12 (('\x0c', 1)) = UNKNOWN
            '\r' => Glyph::CR,           // Glyph 13 (('\r', 1)) = UNKNOWN
            '\u{000e}' => Glyph::SO,     // Glyph 14 (('\x0e', 1)) = UNKNOWN
            '\u{000f}' => Glyph::SI,     // Glyph 15 (('\x0f', 1)) = UNKNOWN
            '\u{0010}' => Glyph::DLE,    // Glyph 16 (('\x10', 1)) = UNKNOWN
            '\u{0011}' => Glyph::DC1,    // Glyph 17 (('\x11', 1)) = UNKNOWN
            '\u{0012}' => Glyph::DC2,    // Glyph 18 (('\x12', 1)) = UNKNOWN
            '\u{0013}' => Glyph::DC3,    // Glyph 19 (('\x13', 1)) = UNKNOWN
            '\u{0014}' => Glyph::DC4,    // Glyph 20 (('\x14', 1)) = UNKNOWN
            '\u{0015}' => Glyph::NAK,    // Glyph 21 (('\x15', 1)) = UNKNOWN
            '\u{0016}' => Glyph::SYN,    // Glyph 22 (('\x16', 1)) = UNKNOWN
            '\u{0017}' => Glyph::ETB,    // Glyph 23 (('\x17', 1)) = UNKNOWN
            '\u{0018}' => Glyph::CAN,    // Glyph 24 (('\x18', 1)) = UNKNOWN
            '\u{0019}' => Glyph::EM,     // Glyph 25 (('\x19', 1)) = UNKNOWN
            '\u{001a}' => Glyph::SUB,    // Glyph 26 (('\x1a', 1)) = UNKNOWN
            '\u{001b}' => Glyph::Escape, // Glyph 27 (('\x1b', 1)) = UNKNOWN
            '\u{001c}' => Glyph::FS,     // Glyph 28 (('\x1c', 1)) = UNKNOWN
            '\u{001d}' => Glyph::GS,     // Glyph 29 (('\x1d', 1)) = UNKNOWN
            '\u{001e}' => Glyph::RS,     // Glyph 30 (('\x1e', 1)) = UNKNOWN
            '\u{001f}' => Glyph::US,     // Glyph 31 (('\x1f', 1)) = UNKNOWN
            ' ' => Glyph::Space,         // Glyph 32 ((' ', 1)) = SPACE
            '!' => Glyph::ExclamationMark, // Glyph 33 (('!', 1)) = EXCLAMATION MARK
            '"' => Glyph::QuotationMark, // Glyph 34 (('"', 1)) = QUOTATION MARK
            '#' => Glyph::NumberSign,    // Glyph 35 (('#', 1)) = NUMBER SIGN
            '$' => Glyph::DollarSign,    // Glyph 36 (('$', 1)) = DOLLAR SIGN
            '%' => Glyph::PercentSign,   // Glyph 37 (('%', 1)) = PERCENT SIGN
            '&' => Glyph::Ampersand,     // Glyph 38 (('&', 1)) = AMPERSAND
            '\'' => Glyph::Apostrophe,   // Glyph 39 (("'", 1)) = APOSTROPHE
            '(' => Glyph::LeftParenthesis, // Glyph 40 (('(', 1)) = LEFT PARENTHESIS
            ')' => Glyph::RightParenthesis, // Glyph 41 ((')', 1)) = RIGHT PARENTHESIS
            '*' => Glyph::Asterisk,      // Glyph 42 (('*', 1)) = ASTERISK
            '+' => Glyph::PlusSign,      // Glyph 43 (('+', 1)) = PLUS SIGN
            ',' => Glyph::Comma,         // Glyph 44 ((',', 1)) = COMMA
            '-' => Glyph::HyphenMinus,   // Glyph 45 (('-', 1)) = HYPHEN-MINUS
            '.' => Glyph::FullStop,      // Glyph 46 (('.', 1)) = FULL STOP
            '/' => Glyph::Solidus,       // Glyph 47 (('/', 1)) = SOLIDUS
            '0' => Glyph::DigitZero,     // Glyph 48 (('0', 1)) = DIGIT ZERO
            '1' => Glyph::DigitOne,      // Glyph 49 (('1', 1)) = DIGIT ONE
            '2' => Glyph::DigitTwo,      // Glyph 50 (('2', 1)) = DIGIT TWO
            '3' => Glyph::DigitThree,    // Glyph 51 (('3', 1)) = DIGIT THREE
            '4' => Glyph::DigitFour,     // Glyph 52 (('4', 1)) = DIGIT FOUR
            '5' => Glyph::DigitFive,     // Glyph 53 (('5', 1)) = DIGIT FIVE
            '6' => Glyph::DigitSix,      // Glyph 54 (('6', 1)) = DIGIT SIX
            '7' => Glyph::DigitSeven,    // Glyph 55 (('7', 1)) = DIGIT SEVEN
            '8' => Glyph::DigitEight,    // Glyph 56 (('8', 1)) = DIGIT EIGHT
            '9' => Glyph::DigitNine,     // Glyph 57 (('9', 1)) = DIGIT NINE
            ':' => Glyph::Colon,         // Glyph 58 ((':', 1)) = COLON
            ';' => Glyph::Semicolon,     // Glyph 59 ((';', 1)) = SEMICOLON
            '<' => Glyph::LessThanSign,  // Glyph 60 (('<', 1)) = LESS-THAN SIGN
            '=' => Glyph::EqualsSign,    // Glyph 61 (('=', 1)) = EQUALS SIGN
            '>' => Glyph::GreaterThanSign, // Glyph 62 (('>', 1)) = GREATER-THAN SIGN
            '?' => Glyph::QuestionMark,  // Glyph 63 (('?', 1)) = QUESTION MARK
            '@' => Glyph::CommercialAt,  // Glyph 64 (('@', 1)) = COMMERCIAL AT
            'A' => Glyph::LatinCapitalLetterA, // Glyph 65 (('A', 1)) = LATIN CAPITAL LETTER A
            'B' => Glyph::LatinCapitalLetterB, // Glyph 66 (('B', 1)) = LATIN CAPITAL LETTER B
            'C' => Glyph::LatinCapitalLetterC, // Glyph 67 (('C', 1)) = LATIN CAPITAL LETTER C
            'D' => Glyph::LatinCapitalLetterD, // Glyph 68 (('D', 1)) = LATIN CAPITAL LETTER D
            'E' => Glyph::LatinCapitalLetterE, // Glyph 69 (('E', 1)) = LATIN CAPITAL LETTER E
            'F' => Glyph::LatinCapitalLetterF, // Glyph 70 (('F', 1)) = LATIN CAPITAL LETTER F
            'G' => Glyph::LatinCapitalLetterG, // Glyph 71 (('G', 1)) = LATIN CAPITAL LETTER G
            'H' => Glyph::LatinCapitalLetterH, // Glyph 72 (('H', 1)) = LATIN CAPITAL LETTER H
            'I' => Glyph::LatinCapitalLetterI, // Glyph 73 (('I', 1)) = LATIN CAPITAL LETTER I
            'J' => Glyph::LatinCapitalLetterJ, // Glyph 74 (('J', 1)) = LATIN CAPITAL LETTER J
            'K' => Glyph::LatinCapitalLetterK, // Glyph 75 (('K', 1)) = LATIN CAPITAL LETTER K
            'L' => Glyph::LatinCapitalLetterL, // Glyph 76 (('L', 1)) = LATIN CAPITAL LETTER L
            'M' => Glyph::LatinCapitalLetterM, // Glyph 77 (('M', 1)) = LATIN CAPITAL LETTER M
            'N' => Glyph::LatinCapitalLetterN, // Glyph 78 (('N', 1)) = LATIN CAPITAL LETTER N
            'O' => Glyph::LatinCapitalLetterO, // Glyph 79 (('O', 1)) = LATIN CAPITAL LETTER O
            'P' => Glyph::LatinCapitalLetterP, // Glyph 80 (('P', 1)) = LATIN CAPITAL LETTER P
            'Q' => Glyph::LatinCapitalLetterQ, // Glyph 81 (('Q', 1)) = LATIN CAPITAL LETTER Q
            'R' => Glyph::LatinCapitalLetterR, // Glyph 82 (('R', 1)) = LATIN CAPITAL LETTER R
            'S' => Glyph::LatinCapitalLetterS, // Glyph 83 (('S', 1)) = LATIN CAPITAL LETTER S
            'T' => Glyph::LatinCapitalLetterT, // Glyph 84 (('T', 1)) = LATIN CAPITAL LETTER T
            'U' => Glyph::LatinCapitalLetterU, // Glyph 85 (('U', 1)) = LATIN CAPITAL LETTER U
            'V' => Glyph::LatinCapitalLetterV, // Glyph 86 (('V', 1)) = LATIN CAPITAL LETTER V
            'W' => Glyph::LatinCapitalLetterW, // Glyph 87 (('W', 1)) = LATIN CAPITAL LETTER W
            'X' => Glyph::LatinCapitalLetterX, // Glyph 88 (('X', 1)) = LATIN CAPITAL LETTER X
            'Y' => Glyph::LatinCapitalLetterY, // Glyph 89 (('Y', 1)) = LATIN CAPITAL LETTER Y
            'Z' => Glyph::LatinCapitalLetterZ, // Glyph 90 (('Z', 1)) = LATIN CAPITAL LETTER Z
            '[' => Glyph::LeftSquareBracket, // Glyph 91 (('[', 1)) = LEFT SQUARE BRACKET
            '\\' => Glyph::ReverseSolidus, // Glyph 92 (('\\', 1)) = REVERSE SOLIDUS
            ']' => Glyph::RightSquareBracket, // Glyph 93 ((']', 1)) = RIGHT SQUARE BRACKET
            '^' => Glyph::CircumflexAccent, // Glyph 94 (('^', 1)) = CIRCUMFLEX ACCENT
            '_' => Glyph::LowLine,       // Glyph 95 (('_', 1)) = LOW LINE
            '`' => Glyph::GraveAccent,   // Glyph 96 (('`', 1)) = GRAVE ACCENT
            'a' => Glyph::LatinSmallLetterA, // Glyph 97 (('a', 1)) = LATIN SMALL LETTER A
            'b' => Glyph::LatinSmallLetterB, // Glyph 98 (('b', 1)) = LATIN SMALL LETTER B
            'c' => Glyph::LatinSmallLetterC, // Glyph 99 (('c', 1)) = LATIN SMALL LETTER C
            'd' => Glyph::LatinSmallLetterD, // Glyph 100 (('d', 1)) = LATIN SMALL LETTER D
            'e' => Glyph::LatinSmallLetterE, // Glyph 101 (('e', 1)) = LATIN SMALL LETTER E
            'f' => Glyph::LatinSmallLetterF, // Glyph 102 (('f', 1)) = LATIN SMALL LETTER F
            'g' => Glyph::LatinSmallLetterG, // Glyph 103 (('g', 1)) = LATIN SMALL LETTER G
            'h' => Glyph::LatinSmallLetterH, // Glyph 104 (('h', 1)) = LATIN SMALL LETTER H
            'i' => Glyph::LatinSmallLetterI, // Glyph 105 (('i', 1)) = LATIN SMALL LETTER I
            'j' => Glyph::LatinSmallLetterJ, // Glyph 106 (('j', 1)) = LATIN SMALL LETTER J
            'k' => Glyph::LatinSmallLetterK, // Glyph 107 (('k', 1)) = LATIN SMALL LETTER K
            'l' => Glyph::LatinSmallLetterL, // Glyph 108 (('l', 1)) = LATIN SMALL LETTER L
            'm' => Glyph::LatinSmallLetterM, // Glyph 109 (('m', 1)) = LATIN SMALL LETTER M
            'n' => Glyph::LatinSmallLetterN, // Glyph 110 (('n', 1)) = LATIN SMALL LETTER N
            'o' => Glyph::LatinSmallLetterO, // Glyph 111 (('o', 1)) = LATIN SMALL LETTER O
            'p' => Glyph::LatinSmallLetterP, // Glyph 112 (('p', 1)) = LATIN SMALL LETTER P
            'q' => Glyph::LatinSmallLetterQ, // Glyph 113 (('q', 1)) = LATIN SMALL LETTER Q
            'r' => Glyph::LatinSmallLetterR, // Glyph 114 (('r', 1)) = LATIN SMALL LETTER R
            's' => Glyph::LatinSmallLetterS, // Glyph 115 (('s', 1)) = LATIN SMALL LETTER S
            't' => Glyph::LatinSmallLetterT, // Glyph 116 (('t', 1)) = LATIN SMALL LETTER T
            'u' => Glyph::LatinSmallLetterU, // Glyph 117 (('u', 1)) = LATIN SMALL LETTER U
            'v' => Glyph::LatinSmallLetterV, // Glyph 118 (('v', 1)) = LATIN SMALL LETTER V
            'w' => Glyph::LatinSmallLetterW, // Glyph 119 (('w', 1)) = LATIN SMALL LETTER W
            'x' => Glyph::LatinSmallLetterX, // Glyph 120 (('x', 1)) = LATIN SMALL LETTER X
            'y' => Glyph::LatinSmallLetterY, // Glyph 121 (('y', 1)) = LATIN SMALL LETTER Y
            'z' => Glyph::LatinSmallLetterZ, // Glyph 122 (('z', 1)) = LATIN SMALL LETTER Z
            '{' => Glyph::LeftCurlyBracket, // Glyph 123 (('{', 1)) = LEFT CURLY BRACKET
            '|' => Glyph::VerticalLine,  // Glyph 124 (('|', 1)) = VERTICAL LINE
            '}' => Glyph::RightCurlyBracket, // Glyph 125 (('}', 1)) = RIGHT CURLY BRACKET
            '~' => Glyph::Tilde,         // Glyph 126 (('~', 1)) = TILDE
            '\u{007f}' => Glyph::Delete, // Glyph 127 (('\x7f', 1)) = DELETE
            'Ç' => Glyph::LatinCapitalLetterCWithCedilla, // Glyph 128 (('Ç', 1)) = LATIN CAPITAL LETTER C WITH CEDILLA
            'ü' => Glyph::LatinSmallLetterUWithDiaeresis, // Glyph 129 (('ü', 1)) = LATIN SMALL LETTER U WITH DIAERESIS
            'é' => Glyph::LatinSmallLetterEWithAcute, // Glyph 130 (('é', 1)) = LATIN SMALL LETTER E WITH ACUTE
            'â' => Glyph::LatinSmallLetterAWithCircumflex, // Glyph 131 (('â', 1)) = LATIN SMALL LETTER A WITH CIRCUMFLEX
            'ä' => Glyph::LatinSmallLetterAWithDiaeresis, // Glyph 132 (('ä', 1)) = LATIN SMALL LETTER A WITH DIAERESIS
            'à' => Glyph::LatinSmallLetterAWithGrave, // Glyph 133 (('à', 1)) = LATIN SMALL LETTER A WITH GRAVE
            'å' => Glyph::LatinSmallLetterAWithRingAbove, // Glyph 134 (('å', 1)) = LATIN SMALL LETTER A WITH RING ABOVE
            'ç' => Glyph::LatinSmallLetterCWithCedilla, // Glyph 135 (('ç', 1)) = LATIN SMALL LETTER C WITH CEDILLA
            'ê' => Glyph::LatinSmallLetterEWithCircumflex, // Glyph 136 (('ê', 1)) = LATIN SMALL LETTER E WITH CIRCUMFLEX
            'ë' => Glyph::LatinSmallLetterEWithDiaeresis, // Glyph 137 (('ë', 1)) = LATIN SMALL LETTER E WITH DIAERESIS
            'è' => Glyph::LatinSmallLetterEWithGrave, // Glyph 138 (('è', 1)) = LATIN SMALL LETTER E WITH GRAVE
            'ï' => Glyph::LatinSmallLetterIWithDiaeresis, // Glyph 139 (('ï', 1)) = LATIN SMALL LETTER I WITH DIAERESIS
            'î' => Glyph::LatinSmallLetterIWithCircumflex, // Glyph 140 (('î', 1)) = LATIN SMALL LETTER I WITH CIRCUMFLEX
            'ì' => Glyph::LatinSmallLetterIWithGrave, // Glyph 141 (('ì', 1)) = LATIN SMALL LETTER I WITH GRAVE
            'Ä' => Glyph::LatinCapitalLetterAWithDiaeresis, // Glyph 142 (('Ä', 1)) = LATIN CAPITAL LETTER A WITH DIAERESIS
            'Å' => Glyph::LatinCapitalLetterAWithRingAbove, // Glyph 143 (('Å', 1)) = LATIN CAPITAL LETTER A WITH RING ABOVE
            'É' => Glyph::LatinCapitalLetterEWithAcute, // Glyph 144 (('É', 1)) = LATIN CAPITAL LETTER E WITH ACUTE
            'æ' => Glyph::LatinSmallLetterAe, // Glyph 145 (('æ', 1)) = LATIN SMALL LETTER AE
            'Æ' => Glyph::LatinCapitalLetterAe, // Glyph 146 (('Æ', 1)) = LATIN CAPITAL LETTER AE
            'ô' => Glyph::LatinSmallLetterOWithCircumflex, // Glyph 147 (('ô', 1)) = LATIN SMALL LETTER O WITH CIRCUMFLEX
            'ö' => Glyph::LatinSmallLetterOWithDiaeresis, // Glyph 148 (('ö', 1)) = LATIN SMALL LETTER O WITH DIAERESIS
            'ò' => Glyph::LatinSmallLetterOWithGrave, // Glyph 149 (('ò', 1)) = LATIN SMALL LETTER O WITH GRAVE
            'û' => Glyph::LatinSmallLetterUWithCircumflex, // Glyph 150 (('û', 1)) = LATIN SMALL LETTER U WITH CIRCUMFLEX
            'ù' => Glyph::LatinSmallLetterUWithGrave, // Glyph 151 (('ù', 1)) = LATIN SMALL LETTER U WITH GRAVE
            'ÿ' => Glyph::LatinSmallLetterYWithDiaeresis, // Glyph 152 (('ÿ', 1)) = LATIN SMALL LETTER Y WITH DIAERESIS
            'Ö' => Glyph::LatinCapitalLetterOWithDiaeresis, // Glyph 153 (('Ö', 1)) = LATIN CAPITAL LETTER O WITH DIAERESIS
            'Ü' => Glyph::LatinCapitalLetterUWithDiaeresis, // Glyph 154 (('Ü', 1)) = LATIN CAPITAL LETTER U WITH DIAERESIS
            'ø' => Glyph::LatinSmallLetterOWithStroke, // Glyph 155 (('ø', 1)) = LATIN SMALL LETTER O WITH STROKE
            '£' => Glyph::PoundSign,                   // Glyph 156 (('£', 1)) = POUND SIGN
            'Ø' => Glyph::LatinCapitalLetterOWithStroke, // Glyph 157 (('Ø', 1)) = LATIN CAPITAL LETTER O WITH STROKE
            '×' => Glyph::MultiplicationSign, // Glyph 158 (('×', 1)) = MULTIPLICATION SIGN
            'ƒ' => Glyph::LatinSmallLetterFWithHook, // Glyph 159 (('ƒ', 1)) = LATIN SMALL LETTER F WITH HOOK
            'á' => Glyph::LatinSmallLetterAWithAcute, // Glyph 160 (('á', 1)) = LATIN SMALL LETTER A WITH ACUTE
            'í' => Glyph::LatinSmallLetterIWithAcute, // Glyph 161 (('í', 1)) = LATIN SMALL LETTER I WITH ACUTE
            'ó' => Glyph::LatinSmallLetterOWithAcute, // Glyph 162 (('ó', 1)) = LATIN SMALL LETTER O WITH ACUTE
            'ú' => Glyph::LatinSmallLetterUWithAcute, // Glyph 163 (('ú', 1)) = LATIN SMALL LETTER U WITH ACUTE
            'ñ' => Glyph::LatinSmallLetterNWithTilde, // Glyph 164 (('ñ', 1)) = LATIN SMALL LETTER N WITH TILDE
            'Ñ' => Glyph::LatinCapitalLetterNWithTilde, // Glyph 165 (('Ñ', 1)) = LATIN CAPITAL LETTER N WITH TILDE
            'ª' => Glyph::FeminineOrdinalIndicator, // Glyph 166 (('ª', 1)) = FEMININE ORDINAL INDICATOR
            'º' => Glyph::MasculineOrdinalIndicator, // Glyph 167 (('º', 1)) = MASCULINE ORDINAL INDICATOR
            '¿' => Glyph::InvertedQuestionMark, // Glyph 168 (('¿', 1)) = INVERTED QUESTION MARK
            '®' => Glyph::RegisteredSign,       // Glyph 169 (('®', 1)) = REGISTERED SIGN
            '¬' => Glyph::NotSign,              // Glyph 170 (('¬', 1)) = NOT SIGN
            '½' => Glyph::VulgarFractionOneHalf, // Glyph 171 (('½', 1)) = VULGAR FRACTION ONE HALF
            '¼' => Glyph::VulgarFractionOneQuarter, // Glyph 172 (('¼', 1)) = VULGAR FRACTION ONE QUARTER
            '¡' => Glyph::InvertedExclamationMark, // Glyph 173 (('¡', 1)) = INVERTED EXCLAMATION MARK
            '«' => Glyph::LeftPointingDoubleAngleQuotationMark, // Glyph 174 (('«', 1)) = LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
            '»' => Glyph::RightPointingDoubleAngleQuotationMark, // Glyph 175 (('»', 1)) = RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
            '░' => Glyph::LightShade, // Glyph 176 (('░', 1)) = LIGHT SHADE
            '▒' => Glyph::MediumShade, // Glyph 177 (('▒', 1)) = MEDIUM SHADE
            '▓' => Glyph::DarkShade,  // Glyph 178 (('▓', 1)) = DARK SHADE
            '│' => Glyph::BoxDrawingsLightVertical, // Glyph 179 (('│', 1)) = BOX DRAWINGS LIGHT VERTICAL
            '┤' => Glyph::BoxDrawingsLightVerticalAndLeft, // Glyph 180 (('┤', 1)) = BOX DRAWINGS LIGHT VERTICAL AND LEFT
            'Á' => Glyph::LatinCapitalLetterAWithAcute, // Glyph 181 (('Á', 1)) = LATIN CAPITAL LETTER A WITH ACUTE
            'Â' => Glyph::LatinCapitalLetterAWithCircumflex, // Glyph 182 (('Â', 1)) = LATIN CAPITAL LETTER A WITH CIRCUMFLEX
            'À' => Glyph::LatinCapitalLetterAWithGrave, // Glyph 183 (('À', 1)) = LATIN CAPITAL LETTER A WITH GRAVE
            '©' => Glyph::CopyrightSign,                // Glyph 184 (('©', 1)) = COPYRIGHT SIGN
            '╣' => Glyph::BoxDrawingsDoubleVerticalAndLeft, // Glyph 185 (('╣', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND LEFT
            '║' => Glyph::BoxDrawingsDoubleVertical, // Glyph 186 (('║', 1)) = BOX DRAWINGS DOUBLE VERTICAL
            '╗' => Glyph::BoxDrawingsDoubleDownAndLeft, // Glyph 187 (('╗', 1)) = BOX DRAWINGS DOUBLE DOWN AND LEFT
            '╝' => Glyph::BoxDrawingsDoubleUpAndLeft, // Glyph 188 (('╝', 1)) = BOX DRAWINGS DOUBLE UP AND LEFT
            '¢' => Glyph::CentSign,                    // Glyph 189 (('¢', 1)) = CENT SIGN
            '¥' => Glyph::YenSign,                     // Glyph 190 (('¥', 1)) = YEN SIGN
            '┐' => Glyph::BoxDrawingsLightDownAndLeft, // Glyph 191 (('┐', 1)) = BOX DRAWINGS LIGHT DOWN AND LEFT
            '└' => Glyph::BoxDrawingsLightUpAndRight, // Glyph 192 (('└', 1)) = BOX DRAWINGS LIGHT UP AND RIGHT
            '┴' => Glyph::BoxDrawingsLightUpAndHorizontal, // Glyph 193 (('┴', 1)) = BOX DRAWINGS LIGHT UP AND HORIZONTAL
            '┬' => Glyph::BoxDrawingsLightDownAndHorizontal, // Glyph 194 (('┬', 1)) = BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
            '├' => Glyph::BoxDrawingsLightVerticalAndRight, // Glyph 195 (('├', 1)) = BOX DRAWINGS LIGHT VERTICAL AND RIGHT
            '─' => Glyph::BoxDrawingsLightHorizontal, // Glyph 196 (('─', 1)) = BOX DRAWINGS LIGHT HORIZONTAL
            '┼' => Glyph::BoxDrawingsLightVerticalAndHorizontal, // Glyph 197 (('┼', 1)) = BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
            'ã' => Glyph::LatinSmallLetterAWithTilde, // Glyph 198 (('ã', 1)) = LATIN SMALL LETTER A WITH TILDE
            'Ã' => Glyph::LatinCapitalLetterAWithTilde, // Glyph 199 (('Ã', 1)) = LATIN CAPITAL LETTER A WITH TILDE
            '╚' => Glyph::BoxDrawingsDoubleUpAndRight, // Glyph 200 (('╚', 1)) = BOX DRAWINGS DOUBLE UP AND RIGHT
            '╔' => Glyph::BoxDrawingsDoubleDownAndRight, // Glyph 201 (('╔', 1)) = BOX DRAWINGS DOUBLE DOWN AND RIGHT
            '╩' => Glyph::BoxDrawingsDoubleUpAndHorizontal, // Glyph 202 (('╩', 1)) = BOX DRAWINGS DOUBLE UP AND HORIZONTAL
            '╦' => Glyph::BoxDrawingsDoubleDownAndHorizontal, // Glyph 203 (('╦', 1)) = BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
            '╠' => Glyph::BoxDrawingsDoubleVerticalAndRight, // Glyph 204 (('╠', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
            '═' => Glyph::BoxDrawingsDoubleHorizontal, // Glyph 205 (('═', 1)) = BOX DRAWINGS DOUBLE HORIZONTAL
            '╬' => Glyph::BoxDrawingsDoubleVerticalAndHorizontal, // Glyph 206 (('╬', 1)) = BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
            '¤' => Glyph::CurrencySign, // Glyph 207 (('¤', 1)) = CURRENCY SIGN
            'ð' => Glyph::LatinSmallLetterEth, // Glyph 208 (('ð', 1)) = LATIN SMALL LETTER ETH
            'Ð' => Glyph::LatinCapitalLetterEth, // Glyph 209 (('Ð', 1)) = LATIN CAPITAL LETTER ETH
            'Ê' => Glyph::LatinCapitalLetterEWithCircumflex, // Glyph 210 (('Ê', 1)) = LATIN CAPITAL LETTER E WITH CIRCUMFLEX
            'Ë' => Glyph::LatinCapitalLetterEWithDiaeresis, // Glyph 211 (('Ë', 1)) = LATIN CAPITAL LETTER E WITH DIAERESIS
            'È' => Glyph::LatinCapitalLetterEWithGrave, // Glyph 212 (('È', 1)) = LATIN CAPITAL LETTER E WITH GRAVE
            'ı' => Glyph::LatinSmallLetterDotlessI, // Glyph 213 (('ı', 1)) = LATIN SMALL LETTER DOTLESS I
            'Í' => Glyph::LatinCapitalLetterIWithAcute, // Glyph 214 (('Í', 1)) = LATIN CAPITAL LETTER I WITH ACUTE
            'Î' => Glyph::LatinCapitalLetterIWithCircumflex, // Glyph 215 (('Î', 1)) = LATIN CAPITAL LETTER I WITH CIRCUMFLEX
            'Ï' => Glyph::LatinCapitalLetterIWithDiaeresis, // Glyph 216 (('Ï', 1)) = LATIN CAPITAL LETTER I WITH DIAERESIS
            '┘' => Glyph::BoxDrawingsLightUpAndLeft, // Glyph 217 (('┘', 1)) = BOX DRAWINGS LIGHT UP AND LEFT
            '┌' => Glyph::BoxDrawingsLightDownAndRight, // Glyph 218 (('┌', 1)) = BOX DRAWINGS LIGHT DOWN AND RIGHT
            '█' => Glyph::FullBlock,                    // Glyph 219 (('█', 1)) = FULL BLOCK
            '▄' => Glyph::LowerHalfBlock, // Glyph 220 (('▄', 1)) = LOWER HALF BLOCK
            '¦' => Glyph::BrokenBar,       // Glyph 221 (('¦', 1)) = BROKEN BAR
            'Ì' => Glyph::LatinCapitalLetterIWithGrave, // Glyph 222 (('Ì', 1)) = LATIN CAPITAL LETTER I WITH GRAVE
            '▀' => Glyph::UpperHalfBlock, // Glyph 223 (('▀', 1)) = UPPER HALF BLOCK
            'Ó' => Glyph::LatinCapitalLetterOWithAcute, // Glyph 224 (('Ó', 1)) = LATIN CAPITAL LETTER O WITH ACUTE
            'ß' => Glyph::LatinSmallLetterSharpS, // Glyph 225 (('ß', 1)) = LATIN SMALL LETTER SHARP S
            'Ô' => Glyph::LatinCapitalLetterOWithCircumflex, // Glyph 226 (('Ô', 1)) = LATIN CAPITAL LETTER O WITH CIRCUMFLEX
            'Ò' => Glyph::LatinCapitalLetterOWithGrave, // Glyph 227 (('Ò', 1)) = LATIN CAPITAL LETTER O WITH GRAVE
            'õ' => Glyph::LatinSmallLetterOWithTilde, // Glyph 228 (('õ', 1)) = LATIN SMALL LETTER O WITH TILDE
            'Õ' => Glyph::LatinCapitalLetterOWithTilde, // Glyph 229 (('Õ', 1)) = LATIN CAPITAL LETTER O WITH TILDE
            'µ' => Glyph::MicroSign,                    // Glyph 230 (('µ', 1)) = MICRO SIGN
            'þ' => Glyph::LatinSmallLetterThorn, // Glyph 231 (('þ', 1)) = LATIN SMALL LETTER THORN
            'Þ' => Glyph::LatinCapitalLetterThorn, // Glyph 232 (('Þ', 1)) = LATIN CAPITAL LETTER THORN
            'Ú' => Glyph::LatinCapitalLetterUWithAcute, // Glyph 233 (('Ú', 1)) = LATIN CAPITAL LETTER U WITH ACUTE
            'Û' => Glyph::LatinCapitalLetterUWithCircumflex, // Glyph 234 (('Û', 1)) = LATIN CAPITAL LETTER U WITH CIRCUMFLEX
            'Ù' => Glyph::LatinCapitalLetterUWithGrave, // Glyph 235 (('Ù', 1)) = LATIN CAPITAL LETTER U WITH GRAVE
            'ý' => Glyph::LatinSmallLetterYWithAcute, // Glyph 236 (('ý', 1)) = LATIN SMALL LETTER Y WITH ACUTE
            'Ý' => Glyph::LatinCapitalLetterYWithAcute, // Glyph 237 (('Ý', 1)) = LATIN CAPITAL LETTER Y WITH ACUTE
            '¯' => Glyph::Macron,                       // Glyph 238 (('¯', 1)) = MACRON
            '´' => Glyph::AcuteAccent,                  // Glyph 239 (('´', 1)) = ACUTE ACCENT
            '\u{00ad}' => Glyph::SoftHyphen,             // Glyph 240 (('\xad', 1)) = SOFT HYPHEN
            '±' => Glyph::PlusMinusSign,                // Glyph 241 (('±', 1)) = PLUS-MINUS SIGN
            '‗' => Glyph::DoubleLowLine,               // Glyph 242 (('‗', 1)) = DOUBLE LOW LINE
            '¾' => Glyph::VulgarFractionThreeQuarters, // Glyph 243 (('¾', 1)) = VULGAR FRACTION THREE QUARTERS
            '¶' => Glyph::PilcrowSign,                 // Glyph 244 (('¶', 1)) = PILCROW SIGN
            '§' => Glyph::SectionSign,                 // Glyph 245 (('§', 1)) = SECTION SIGN
            '÷' => Glyph::DivisionSign,                // Glyph 246 (('÷', 1)) = DIVISION SIGN
            '¸' => Glyph::Cedilla,                     // Glyph 247 (('¸', 1)) = CEDILLA
            '°' => Glyph::DegreeSign,                  // Glyph 248 (('°', 1)) = DEGREE SIGN
            '¨' => Glyph::Diaeresis,                   // Glyph 249 (('¨', 1)) = DIAERESIS
            '·' => Glyph::MiddleDot,                   // Glyph 250 (('·', 1)) = MIDDLE DOT
            '¹' => Glyph::SuperscriptOne,              // Glyph 251 (('¹', 1)) = SUPERSCRIPT ONE
            '³' => Glyph::SuperscriptThree,            // Glyph 252 (('³', 1)) = SUPERSCRIPT THREE
            '²' => Glyph::SuperscriptTwo,              // Glyph 253 (('²', 1)) = SUPERSCRIPT TWO
            '■' => Glyph::BlackSquare,                // Glyph 254 (('■', 1)) = BLACK SQUARE
            '\u{00a0}' => Glyph::NoBreakSpace,          // Glyph 255 (('\xa0', 1)) = NO-BREAK SPACE
            '☺' => Glyph::SOH,                          // Glyph 001 (('☺', 1)) = WHITE SMILING FACE
            '☻' => Glyph::STX,                          // Glyph 002 (('☻', 1)) = BLACK SMILING FACE
            '♥' => Glyph::ETX,                          // Glyph 003 (('♥', 1)) = BLACK HEART SUIT
            '♦' => Glyph::EOT,                          // Glyph 004 (('♦', 1)) = BLACK DIAMOND SUIT
            '♣' => Glyph::ENQ,                          // Glyph 005 (('♣', 1)) = BLACK CLUB SUIT
            '♠' => Glyph::ACK,                          // Glyph 006 (('♠', 1)) = BLACK SPADE SUIT
            '•' => Glyph::BEL,                          // Glyph 007 (('•', 1)) = BULLET
            '◘' => Glyph::BS,                           // Glyph 008 (('◘', 1)) = INVERSE BULLET
            '○' => Glyph::HT,                           // Glyph 009 (('○', 1)) = WHITE CIRCLE
            '◙' => Glyph::LF,                           // Glyph 010 (('◙', 1)) = INVERSE WHITE CIRCLE
            '♂' => Glyph::VT,                           // Glyph 011 (('♂', 1)) = MALE SIGN
            '♀' => Glyph::FF,                           // Glyph 012 (('♀', 1)) = FEMALE SIGN
            '♪' => Glyph::CR,                           // Glyph 013 (('♪', 1)) = EIGHTH NOTE
            '♫' => Glyph::SO,                           // Glyph 014 (('♫', 1)) = BEAMED EIGHTH NOTES
            '☼' => Glyph::SI,                           // Glyph 015 (('☼', 1)) = WHITE SUN WITH RAYS
            '►' => Glyph::DLE,                          // Glyph 016 (('►', 1)) = BLACK RIGHT-POINTING ARROW
            '◄' => Glyph::DC1,                          // Glyph 017 (('◄', 1)) = BLACK LEFT-POINTING ARROW
            '↕' => Glyph::DC2,                          // Glyph 018 (('↕', 1)) = UP DOWN ARROW
            '‼' => Glyph::DC3,                          // Glyph 019 (('‼', 1)) = DOUBLE EXCLAMATION MARK
            '▬' => Glyph::SYN,                          // Glyph 022 (('▬', 1)) = BLACK RECTANGLE
            '↨' => Glyph::ETB,                          // Glyph 023 (('↨', 1)) = UP DOWN ARROW WITH BASE
            '↑' => Glyph::CAN,                          // Glyph 024 (('↑', 1)) = UPWARDS ARROW
            '↓' => Glyph::EM,                           // Glyph 025 (('↓', 1)) = DOWNWARDS ARROW
            '→' => Glyph::SUB,                          // Glyph 026 (('→', 1)) = RIGHTWARDS ARROW
            '←' => Glyph::Escape,                       // Glyph 027 (('←', 1)) = LEFTWARDS ARROW
            '∟' => Glyph::FS,                           // Glyph 028 (('∟', 1)) = RIGHT ANDLE
            '↔' => Glyph::GS,                           // Glyph 029 (('↔', 1)) = LEFT RIGHT ARROW
            '▲' => Glyph::RS,                           // Glyph 030 (('▲', 1)) = BLACK UP POINTING ARROW
            '▼' => Glyph::US,                           // Glyph 031 (('▼', 1)) = BLACK DOWN POINTING ARROW
            _ => Glyph::QuestionMark,                   // Map unknown chars to ?
        }
    }

    /// Take a CodePage 850 byte
    pub fn from_byte(byte: u8) -> Glyph {
        unsafe { ::core::mem::transmute(byte) }
    }

    pub fn pixels(self, row: usize) -> u8 {
        let index = ((self as usize) * FONT_HEIGHT) + row;
        FONT_DATA[index]
    }
}
