use insta::assert_display_snapshot;
use layered_amount::{Amount, AmountResolver};
use layered_nlp as nlp;

use super::{Currency, Money, MoneyResolver};

#[test]
fn out_of_the_box() {
    let line = nlp::create_line_from_string("What's $20,000 times 12 months?");

    let display = nlp::debug_line(&line)
        // Tokens which can be represented as a single `char` get the `char` attribute for convenience with [nlp::x::attr_eq].
        .with::<char>()
        // Most basic tokenization step tries to split the text into pieces.
        .with::<nlp::TextTag>();

    assert_display_snapshot!(display, @r###"
    What's     $  20  ,  000     times     12     months  ?
            ╰' '
               ╰'$'
                      ╰','
                              ╰' '
                                        ╰' '
                                               ╰' '
                                                          ╰'?'
    ╰────╯WORD
            ╰SPACE
               ╰SYMB
                  ╰╯NATN
                      ╰PUNC
                         ╰─╯NATN
                              ╰SPACE
                                 ╰───╯WORD
                                        ╰SPACE
                                           ╰╯NATN
                                               ╰SPACE
                                                  ╰────╯WORD
                                                          ╰PUNC
    "###);
}

#[test]
fn it_works() {
    let line = nlp::create_line_from_string("$20,000")
        .run(&AmountResolver::new(vec![','], '.'))
        .run(&MoneyResolver);

    let display = nlp::debug_line(&line)
        // inputs
        .with::<char>()
        .with::<Amount>()
        // output
        .with::<Money>();

    assert_display_snapshot!(display, @r###"
    $  20  ,  000
    ╰'$'
           ╰','
       ╰────────╯Amount(20000)
    ╰───────────╯Money(USD, Amount(20000))
    "###);
}
