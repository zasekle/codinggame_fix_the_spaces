use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct Attempt {
    str: String,
    curr_idx: usize,
    remaining_words: Vec::<String>
}

impl Hash for Attempt {
    fn hash<H>(&self, h: &mut H)
        where
            H: Hasher,
    {
        self.str.hash(h)
    }
}

impl PartialEq for Attempt {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl Eq for Attempt {}

fn main() {
    let original = "LOREMIPSUMDOLORSITAMETCONSECTETURADIPISCINGELITSEDDOEIUSMODTEMPORINCIDIDUNTUTLABOREETDOLOREMAGNAALIQUAESTLIBEROMOLESTIEACCUMSANDUIACRISUSINCEPTOSHABITASSEHABITASSEFINIBUSAORNAREINTERDUMVITAEHIMENAEOSAUGUEPHASELLUSSAPIENSITSOCIOSQUPHARETRAFINIBUSMORBIALIQUETTINCIDUNTCLASSESTDIAMHABITASSEFEUGIATMAGNAPROINMAURISVIVAMUSTINCIDUNTALIQUETNIBHALIQUAMACUBILIATELLUSLEOFELISALIQUAMVOLUTPATANTENAMVITAEDOLORPRETIUMTELLUSGRAVIDACONGUEALIQUETFAMESCONDIMENTUMNUNCNULLADIAMPRAESENTEGESTASTORTORALIQUETCONDIMENTUMFUSCEVULPUTATEPURUSINTERDUMMAURISULTRICESPULVINARFINIBUSNEQUELIGULALUCTUSNULLAMDAPIBUSBLANDITACDOLORVEHICULAMOLESTIELAOREETPLATEATINCIDUNTSAPIENPURUSNISLMOLLISIPSUMMINISIVELITHENDRERITSEDELEMENTUMCURABITURMAURISPHARETRAUTARCUSUSCIPITQUISVENENATISFAMESFEUGIATVELITNETUSBLANDITCONUBIAVELPROINPOSUEREVEHICULALAOREETCRASSEMPERMAXIMUSFINIBUS";
    let words_set_str = Vec::<&str>::from(
        ["PRETIUM", "DOLORE", "VEHICULA", "FINIBUS", "ET", "ALIQUAM", "VELIT", "MOLESTIE", "ALIQUET", "CONDIMENTUM", "MAURIS", "CONDIMENTUM", "PROIN", "LIBERO", "CRAS", "MI", "AC", "TINCIDUNT", "NISL", "HABITASSE", "VITAE", "EST", "MOLESTIE", "VEHICULA", "MOLLIS", "FAMES", "AMET", "NAM", "UT", "CLASS", "ANTE", "HIMENAEOS", "DIAM", "NUNC", "BLANDIT", "NETUS", "NIBH", "IPSUM", "SED", "A", "QUIS", "NEQUE", "SAPIENPURUS", "ULTRICES", "AC", "LOREM", "FAMES", "UT", "LAOREET", "SUSCIPIT", "ALIQUET", "VULPUTATE", "SEMPER", "ALIQUAM", "FELIS", "TELLUS", "NULLA", "TINCIDUNT", "INTERDUM", "CONGUE", "ALIQUET", "MAXIMUS", "NISI", "LEO", "POSUERE", "LIGULA", "FEUGIAT", "HABITASSE", "RISUS", "TEMPOR", "INTERDUM", "PROIN", "ADIPISCING", "FINIBUS", "PHARETRA", "TORTOR", "ELEMENTUM", "VELIT", "INCEPTOS", "VEL", "ELIT", "DOLOR", "A", "DO", "CONSECTETUR", "LUCTUS", "BLANDIT", "VENENATIS", "FEUGIAT", "MAGNA", "TELLUSGRAVIDA", "MAURIS", "VITAE", "PHARETRA", "SAPIEN", "ALIQUA", "CUBILIA", "DOLOR", "PULVINAR", "DUI", "LAOREET", "FINIBUS", "PURUS", "DIAM", "MORBI", "EIUSMOD", "PLATEA", "MAGNA", "CURABITUR", "ORNARE", "PRAESENT", "DOLOR", "NULLAM", "EGESTAS", "TINCIDUNT", "LABORE", "IPSUM", "CONUBIA", "FUSCE", "INCIDIDUNT", "SIT", "FINIBUS", "MAURIS", "VOLUTPAT", "ARCU", "AUGUE", "HENDRERIT", "SOCIOSQU", "HABITASSE", "SIT", "EST", "ACCUMSAN", "PHASELLUS", "SED", "ALIQUET", "VIVAMUS", "DAPIBUS"]
    );

    let mut words_set = Vec::<String>::new();

    for str in words_set_str {
        words_set.push(String::from(str));
    }

    let mut attempts = HashSet::<Attempt>::new();
    attempts.insert(
        Attempt{
            str: String::new(),
            curr_idx: 0,
            remaining_words: words_set
        }
    );

    let mut something_changed = true;
    let mut attempt_num = 0;
    while something_changed {
        something_changed = false;
        let mut next_attempts = HashSet::<Attempt>::new();

        println!("ATTEMPT {attempt_num}");
        for attempt in &attempts {
            println!("{}", attempt.str);
        }

        for attempt in attempts {
            //while !attempts.is_empty() {

            if attempt.curr_idx > original.len()-1 {
                next_attempts.insert(attempt);
            } else {

                for w in &attempt.remaining_words {
                    if w.len() <= original.len() - attempt.curr_idx {
                        let string_match = &original[attempt.curr_idx..attempt.curr_idx+w.len()];
                        if string_match == w {

                            something_changed = true;
                            let mut attempt_str = String::from(attempt.str.clone());

                            if !attempt_str.is_empty() {
                                attempt_str.push(' ');
                            }

                            attempt_str.push_str(w);
                            let attempt_curr_idx = attempt.curr_idx + w.len();

                            let mut attempt_remaining_words = attempt.remaining_words.clone();

                            let mut idx_to_remove = 0;
                            for (i, s) in attempt_remaining_words.iter().enumerate() {
                                if s == w {
                                    idx_to_remove = i;
                                    break;
                                }
                            }
                            attempt_remaining_words.remove(idx_to_remove);

                            next_attempts.insert(
                                Attempt{
                                    str: attempt_str,
                                    curr_idx: attempt_curr_idx,
                                    remaining_words: attempt_remaining_words
                                }
                            );
                        }
                    }
                }
            }
        }

        attempts = next_attempts;
        attempt_num += 1;
    }

    if attempts.len() != 1 {
        println!("Unsolvable");
    } else {
        println!("{}", attempts.iter().next().expect("str").str);
    }

}