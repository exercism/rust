use std::collections::HashMap;

use parallel_letter_frequency as frequency;

#[test]
fn no_texts() {
    assert_eq!(frequency::frequency(&[], 4), HashMap::new());
}

#[test]
#[ignore]
fn one_text_with_one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(frequency::frequency(&["a"], 4), hm);
}

#[test]
#[ignore]
fn one_text_with_multiple_letters() {
    let mut hm = HashMap::new();
    hm.insert('b', 2);
    hm.insert('c', 3);
    hm.insert('d', 1);
    assert_eq!(frequency::frequency(&["bbcccd"], 4), hm);
}

#[test]
#[ignore]
fn two_texts_with_one_letter() {
    let mut hm = HashMap::new();
    hm.insert('e', 1);
    hm.insert('f', 1);
    assert_eq!(frequency::frequency(&["e", "f"], 4), hm);
}

#[test]
#[ignore]
fn two_texts_with_multiple_letters() {
    let mut hm = HashMap::new();
    hm.insert('g', 2);
    hm.insert('h', 3);
    hm.insert('i', 1);
    assert_eq!(frequency::frequency(&["ggh", "hhi"], 4), hm);
}

#[test]
#[ignore]
fn case_insensitivity() {
    let mut hm = HashMap::new();
    hm.insert('a', 2);
    assert_eq!(frequency::frequency(&["aA"], 4), hm);
}

#[test]
#[ignore]
fn many_empty_lines() {
    let v = vec![""; 1000];
    assert_eq!(frequency::frequency(&v[..], 4), HashMap::new());
}

#[test]
#[ignore]
fn ignore_whitespace() {
    let v = ["   ", "\t", "\r\n"];
    assert_eq!(frequency::frequency(&v[..], 4), HashMap::new());
}

#[test]
#[ignore]
fn many_times_same_text() {
    let v = vec!["abc"; 1000];
    let mut hm = HashMap::new();
    hm.insert('a', 1000);
    hm.insert('b', 1000);
    hm.insert('c', 1000);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}

#[test]
#[ignore]
fn punctuation_doesnt_count() {
    assert!(!frequency::frequency(&WILHELMUS, 4).contains_key(&','));
}

#[test]
#[ignore]
fn numbers_dont_count() {
    assert!(!frequency::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
}

#[test]
#[ignore]
fn unicode_letters() {
    let mut hm = HashMap::new();
    hm.insert('本', 1);
    hm.insert('φ', 1);
    hm.insert('ほ', 1);
    hm.insert('ø', 1);
    let v = ["本", "φ", "ほ", "ø"];
    assert_eq!(frequency::frequency(&v, 4), hm);
}

#[test]
#[ignore]
fn all_three_anthems_1_worker() {
    let mut v = Vec::new();
    // These constants can be found under the last test if you wish to see them
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}

#[test]
#[ignore]
fn all_three_anthems_3_workers() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}

#[test]
#[ignore]
fn non_integer_multiple_of_threads() {
    let v = vec!["abc"; 999];
    let mut hm = HashMap::new();
    hm.insert('a', 999);
    hm.insert('b', 999);
    hm.insert('c', 999);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}

#[test]
#[ignore]
fn large_texts() {
    let expected: HashMap<char, usize> = [
        ('a', 845),
        ('b', 155),
        ('c', 278),
        ('d', 359),
        ('e', 1143),
        ('f', 222),
        ('g', 187),
        ('h', 507),
        ('i', 791),
        ('j', 12),
        ('k', 67),
        ('l', 423),
        ('m', 288),
        ('n', 833),
        ('o', 791),
        ('p', 197),
        ('q', 8),
        ('r', 432),
        ('s', 700),
        ('t', 1043),
        ('u', 325),
        ('v', 111),
        ('w', 223),
        ('x', 7),
        ('y', 251),
    ]
    .into_iter()
    .collect();

    assert_eq!(frequency::frequency(&DOSTOEVSKY, 4), expected);
}

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

// Dutch national anthem
const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];

// American national anthem
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

const DOSTOEVSKY: [&str; 4] = [
    r#"
    I am a sick man.... I am a spiteful man. I am an unattractive man.
    I believe my liver is diseased. However, I know nothing at all about my disease, and do not
    know for certain what ails me. I don't consult a doctor for it,
    and never have, though I have a respect for medicine and doctors.
    Besides, I am extremely superstitious, sufficiently so to respect medicine,
    anyway (I am well-educated enough not to be superstitious, but I am superstitious).
    No, I refuse to consult a doctor from spite.
    That you probably will not understand. Well, I understand it, though.
    Of course, I can't explain who it is precisely that I am mortifying in this case by my spite:
    I am perfectly well aware that I cannot "pay out" the doctors by not consulting them;
    I know better than anyone that by all this I am only injuring myself and no one else.
    But still, if I don't consult a doctor it is from spite.
    My liver is bad, well - let it get worse!
    I have been going on like that for a long time - twenty years. Now I am forty.
    I used to be in the government service, but am no longer.
    I was a spiteful official. I was rude and took pleasure in being so.
    I did not take bribes, you see, so I was bound to find a recompense in that, at least.
    (A poor jest, but I will not scratch it out. I wrote it thinking it would sound very witty;
    but now that I have seen myself that I only wanted to show off in a despicable way -
    I will not scratch it out on purpose!) When petitioners used to come for
    information to the table at which I sat, I used to grind my teeth at them,
    and felt intense enjoyment when I succeeded in making anybody unhappy.
    I almost did succeed. For the most part they were all timid people - of course,
    they were petitioners. But of the uppish ones there was one officer in particular
    I could not endure. He simply would not be humble, and clanked his sword in a disgusting way.
    I carried on a feud with him for eighteen months over that sword. At last I got the better of him.
    He left off clanking it. That happened in my youth, though. But do you know,
    gentlemen, what was the chief point about my spite? Why, the whole point,
    the real sting of it lay in the fact that continually, even in the moment of the acutest spleen,
    I was inwardly conscious with shame that I was not only not a spiteful but not even an embittered man,
    that I was simply scaring sparrows at random and amusing myself by it.
    I might foam at the mouth, but bring me a doll to play with, give me a cup of tea with sugar in it,
    and maybe I should be appeased. I might even be genuinely touched,
    though probably I should grind my teeth at myself afterwards and lie awake at night with shame for
    months after. That was my way. I was lying when I said just now that I was a spiteful official.
    I was lying from spite. I was simply amusing myself with the petitioners and with the officer,
    and in reality I never could become spiteful. I was conscious every moment in myself of many,
    very many elements absolutely opposite to that. I felt them positively swarming in me,
    these opposite elements. I knew that they had been swarming in me all my life and craving some outlet from me,
    but I would not let them, would not let them, purposely would not let them come out.
    They tormented me till I was ashamed: they drove me to convulsions and - sickened me, at last,
    how they sickened me!
    "#,
    r#"
    Gentlemen, I am joking, and I know myself that my jokes are not brilliant,
    but you know one can take everything as a joke. I am, perhaps, jesting against the grain.
    Gentlemen, I am tormented by questions; answer them for me. You, for instance, want to cure men of their
    old habits and reform their will in accordance with science and good sense.
    But how do you know, not only that it is possible, but also that it is
    desirable to reform man in that way? And what leads you to the conclusion that man's
    inclinations need reforming? In short, how do you know that such a reformation will be a benefit to man?
    And to go to the root of the matter, why are you so positively convinced that not to act against
    his real normal interests guaranteed by the conclusions of reason and arithmetic is certainly always
    advantageous for man and must always be a law for mankind? So far, you know,
    this is only your supposition. It may be the law of logic, but not the law of humanity.
    You think, gentlemen, perhaps that I am mad? Allow me to defend myself. I agree that man
    is pre-eminently a creative animal, predestined to strive consciously for an object and to engage in engineering -
    that is, incessantly and eternally to make new roads, wherever
    they may lead. But the reason why he wants sometimes to go off at a tangent may just be that he is
    predestined to make the road, and perhaps, too, that however stupid the "direct"
    practical man may be, the thought sometimes will occur to him that the road almost always does lead
    somewhere, and that the destination it leads to is less important than the process
    of making it, and that the chief thing is to save the well-conducted child from despising engineering,
    and so giving way to the fatal idleness, which, as we all know,
    is the mother of all the vices. Man likes to make roads and to create, that is a fact beyond dispute.
    But why has he such a passionate love for destruction and chaos also?
    Tell me that! But on that point I want to say a couple of words myself. May it not be that he loves
    chaos and destruction (there can be no disputing that he does sometimes love it)
    because he is instinctively afraid of attaining his object and completing the edifice he is constructing?
    Who knows, perhaps he only loves that edifice from a distance, and is by no means
    in love with it at close quarters; perhaps he only loves building it and does not want to live in it,
    but will leave it, when completed, for the use of les animaux domestiques -
    such as the ants, the sheep, and so on. Now the ants have quite a different taste.
    They have a marvellous edifice of that pattern which endures for ever - the ant-heap.
    With the ant-heap the respectable race of ants began and with the ant-heap they will probably end,
    which does the greatest credit to their perseverance and good sense. But man is a frivolous and
    incongruous creature, and perhaps, like a chess player, loves the process of the game, not the end of it.
    And who knows (there is no saying with certainty), perhaps the only goal on earth
    to which mankind is striving lies in this incessant process of attaining, in other words,
    in life itself, and not in the thing to be attained, which must always be expressed as a formula,
    as positive as twice two makes four, and such positiveness is not life, gentlemen,
    but is the beginning of death.
    "#,
    r#"
    But these are all golden dreams. Oh, tell me, who was it first announced,
    who was it first proclaimed, that man only does nasty things because he does not know his own interests;
    and that if he were enlightened, if his eyes were opened to his real normal interests,
    man would at once cease to do nasty things, would at once become good and noble because,
    being enlightened and understanding his real advantage, he would see his own advantage in the
    good and nothing else, and we all know that not one man can, consciously, act against his own interests,
    consequently, so to say, through necessity, he would begin doing good? Oh, the babe! Oh, the pure,
    innocent child! Why, in the first place, when in all these thousands of years has there been a time
    when man has acted only from his own interest? What is to be done with the millions of facts that bear
    witness that men, consciously, that is fully understanding their real interests, have left them in the
    background and have rushed headlong on another path, to meet peril and danger,
    compelled to this course by nobody and by nothing, but, as it were, simply disliking the beaten track,
    and have obstinately, wilfully, struck out another difficult, absurd way, seeking it almost in the darkness.
    So, I suppose, this obstinacy and perversity were pleasanter to them than any advantage....
    Advantage! What is advantage? And will you take it upon yourself to define with perfect accuracy in what the
    advantage of man consists? And what if it so happens that a man's advantage, sometimes, not only may,
    but even must,  consist in his desiring in certain cases what is harmful to himself and not advantageous.
    And if so, if there can be such a case, the whole principle falls into dust. What do you think -
    are there such cases? You laugh; laugh away, gentlemen, but only answer me: have man's advantages been
    reckoned up with perfect certainty? Are there not some which not only have not been included but cannot
    possibly be included under any classification? You see, you gentlemen have, to the best of my knowledge,
    taken your whole register of human advantages from the averages of statistical figures and
    politico-economical formulas. Your advantages are prosperity, wealth, freedom, peace - and so on, and so on.
    So that the man who should, for instance, go openly and knowingly in opposition to all that list would to your thinking,
    and indeed mine, too, of course, be an obscurantist or an absolute madman: would not he? But, you know, this is
    what is surprising: why does it so happen that all these statisticians,  sages and lovers of humanity,
    when they reckon up human advantages invariably leave out one? They don't even take it into their reckoning
    in the form in which it should be taken, and the whole reckoning depends upon that. It would be no greater matter,
    they would simply have to take it, this advantage, and add it to the list. But the trouble is, that this strange
    advantage does not fall under any classification and is not in place in any list. I have a friend for instance ...
    Ech! gentlemen, but of course he is your friend, too; and indeed there is no one, no one to whom he is not a friend!
    "#,
    r#"
    Yes, but here I come to a stop! Gentlemen, you must excuse me for being over-philosophical;
    it's the result of forty years underground! Allow me to indulge my fancy. You see, gentlemen, reason is an excellent thing,
    there's no disputing that, but reason is nothing but reason and satisfies only the rational side of man's nature,
    while will is a manifestation of the whole life, that is, of the whole human life including reason and all the impulses.
    And although our life, in this manifestation of it, is often worthless, yet it is life and not simply extracting square roots.
    Here I, for instance, quite naturally want to live, in order to satisfy all my capacities for life, and not simply my capacity
    for reasoning, that is, not simply one twentieth of my capacity for life. What does reason know? Reason only knows what it has
    succeeded in learning (some things, perhaps, it will never learn; this is a poor comfort, but why not say so frankly?)
    and human nature acts as a whole, with everything that is in it, consciously or unconsciously, and, even it if goes wrong, it lives.
    I suspect, gentlemen, that you are looking at me with compassion; you tell me again that an enlightened and developed man,
    such, in short, as the future man will be, cannot consciously desire anything disadvantageous to himself, that that can be proved mathematically.
    I thoroughly agree, it can - by mathematics. But I repeat for the hundredth time, there is one case, one only, when man may consciously, purposely,
    desire what is injurious to himself, what is stupid, very stupid - simply in order to have the right to desire for himself even what is very stupid
    and not to be bound by an obligation to desire only what is sensible. Of course, this very stupid thing, this caprice of ours, may be in reality,
    gentlemen, more advantageous for us than anything else on earth, especially in certain cases. And in particular it may be more advantageous than
    any advantage even when it does us obvious harm, and contradicts the soundest conclusions of our reason concerning our advantage -
    for in any circumstances it preserves for us what is most precious and most important - that is, our personality, our individuality.
    Some, you see, maintain that this really is the most precious thing for mankind; choice can, of course, if it chooses, be in agreement
    with reason; and especially if this be not abused but kept within bounds. It is profitable and some- times even praiseworthy.
    But very often, and even most often, choice is utterly and stubbornly opposed to reason ... and ... and ... do you know that that,
    too, is profitable, sometimes even praiseworthy? Gentlemen, let us suppose that man is not stupid. (Indeed one cannot refuse to suppose that,
    if only from the one consideration, that, if man is stupid, then who is wise?) But if he is not stupid, he is monstrously ungrateful!
    Phenomenally ungrateful. In fact, I believe that the best definition of man is the ungrateful biped. But that is not all, that is not his worst defect;
    his worst defect is his perpetual moral obliquity, perpetual - from the days of the Flood to the Schleswig-Holstein period.
    "#,
];
