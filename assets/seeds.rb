################################################################################
# Destroy Database
################################################################################

# ActivationCostAgenda.destroy_all
# VariantAbility.destroy_all
# AbilityCost.destroy_all
# VariantType.destroy_all
# VariantTechSlot.destroy_all
# TechAbility.destroy_all
# Tech.destroy_all
# Type.destroy_all
# Agenda.destroy_all
# Ability.destroy_all
# Variant.destroy_all
# Operator.destroy_all
# TechCategory.destroy_all
# TechType.destroy_all
# Century.destroy_all

################################################################################
# Create Operators
################################################################################

@mori = Operator.create!(name: 'Mori')
@angela_storm = Operator.create!(name: 'Angela Storm')
@ben_storm = Operator.create!(name: 'Ben Storm')
@akane = Operator.create!(name: 'Akane')
@the_pale_man = Operator.create!(name: 'The Pale Man')
@mia_star = Operator.create!(name: 'Mia Star')
@augur = Operator.create!(name: 'Augur')
@man_bon_hwa = Operator.create!(name: 'Man Bon-Hwa')
@big_boy = Operator.create!(name: 'Big Boy')
@little_boy = Operator.create!(name: 'Little Boy')
@sourayah = Operator.create!(name: 'Sourayah')
@aran = Operator.create!(name: 'Aran')
@layla = Operator.create!(name: 'Layla')
@hamdi = Operator.create!(name: 'Hamdi')
@marwa = Operator.create!(name: 'Marwa')
@dr_koh = Operator.create!(name: 'Dr. Koh')
@captain_shortspear = Operator.create!(name: 'Captain Shortspear')
@soren = Operator.create!(name: 'Soren')
@the_gray_baron = Operator.create!(name: 'The Gray Baron')
@dr_rosu = Operator.create!(name: 'Dr. Roşu')
@mr_rosu = Operator.create!(name: 'Mr. Roşu')
@striker1 = Operator.create!(name: 'Striker')
@zeev = Operator.create!(name: "Ze'ev")
@emory_iv = Operator.create!(name: 'Emory IV')

################################################################################
# Create Centuries
################################################################################

@_1800 = Century.create!(name: 1800)
@_1900 = Century.create!(name: 1900)
@_2000 = Century.create!(name: 2000)
@_2100 = Century.create!(name: 2100)
@_2200 = Century.create!(name: 2200)
@_2300 = Century.create!(name: 2300)

################################################################################
# Create Variants
################################################################################

@v_mori_the_piercer = Variant.create!(
  operator_id: @mori.id,
  century_id: @_1800.id,
  name: "The Piercer",
  sex: 0,
  defense: 1,
  flavour_text: "I live by a code. The code says nothing about time travel.",
  image: "operators/mori_the_piercer.jpg"
)

@v_angela_storm = Variant.create!(
  century_id: @_2000.id,
  operator_id: @angela_storm.id,
  sex: 3,
  defense: 3,
  flavour_text: "Ask me if this is the worst I can do. Please.",
  image: "operators/angela_storm.jpg"
)

@v_ben_storm = Variant.create!(
  century_id: @_2000.id,
  operator_id: @ben_storm.id,
  sex: 0,
  defense: 2,
  flavour_text: "You think my ambitions are twisted? You haven't met my twin sister.",
  image: "operators/ben_storm.jpg"
)

@v_ben_storm_the_looper = Variant.create!(
  operator_id: @ben_storm.id,
  century_id: @_2000.id,
  name: "The Looper",
  sex: 0,
  defense: 1,
  flavour_text: "Koh spliced away one too many of my inhibition genes. He'll pay, together with anyone else in my runway.",
  image: "operators/ben_storm_the_looper.jpg"
)

@v_ben_storm_the_transcended = Variant.create!(
  operator_id: @ben_storm.id,
  century_id: @_2100.id,
  name: "The Transcended",
  sex: 2,
  defense: 2,
  flavour_text: "Join me outside the box, dead and alive.",
  image: "operators/ben_storm_the_transcended.jpg"
)

@v_akane = Variant.create!(
  century_id: @_2100.id,
  operator_id: @akane.id,
  sex: 1,
  defense: 3,
  flavour_text: "'Not like this' I asked, but he had two needs of me.",
  image: "operators/akane.jpg"
)

@v_the_pale_man = Variant.create!(
  century_id: @_2200.id,
  operator_id: @the_pale_man.id,
  sex: 2,
  defense: 2,
  flavour_text: "I do not have higher functions.",
  image: "operators/the_pale_man.jpg"
)

@v_mia_star = Variant.create!(
  century_id: @_2300.id,
  operator_id: @mia_star.id,
  sex: 1,
  defense: 2,
  flavour_text: "My mind is a star and my palms are solar sails.",
  image: "operators/mia_star.jpg"
)

@v_augur_stream_phase_shaman = Variant.create!(
  operator_id: @augur.id,
  century_id: @_2300.id,
  name: "Stream-Phase Shaman",
  sex: 0,
  defense: 1,
  flavour_text: "The dead call to me from the void between the streams. I feed the living to them.",
  image: "operators/augur_stream_phase_shaman.jpg"
)

@v_man_bon_hwa = Variant.create!(
  century_id: @_2300.id,
  operator_id: @man_bon_hwa.id,
  sex: 0,
  defense: 2,
  flavour_text: "I adore women, the pinnacle of their free will is slavery. All they need is the right master.",
  image: "operators/man_bon_hwa.jpg"
)

@v_big_boy = Variant.create!(
  century_id: @_2300.id,
  operator_id: @big_boy.id,
  sex: 2,
  defense: 4,
  flavour_text: "Make room. Or don't.",
  image: "operators/big_boy.jpg"
)

@v_little_boy = Variant.create!(
  century_id: @_2100.id,
  operator_id: @little_boy.id,
  sex: 2,
  defense: 1,
  flavour_text: "Fasten your seatbelts.",
  image: "operators/little_boy.jpg"
)

@v_sourayah_the_bad_prophet = Variant.create!(
  operator_id: @sourayah.id,
  century_id: @_1800.id,
  name: "The Bad Prophet",
  sex: 1,
  defense: 3,
  flavour_text: "They all think they need me, freely willing to sacrifice so much. This is how my gift works.",
  image: "operators/sourayah_the_bad_prophet.jpg"
)

@v_aran = Variant.create!(
  century_id: @_1900.id,
  operator_id: @aran.id,
  sex: 0,
  defense: 1,
  flavour_text: "I am like the ham'sin: desert wind, dust, and pain.",
  image: "operators/aran.jpg"
)

@v_layla = Variant.create!(
  century_id: @_2000.id,
  operator_id: @layla.id,
  sex: 1,
  defense: 2,
  flavour_text: "If the desert sand does not drink you, I will.",
  image: "operators/layla.jpg"
)

@v_hamdi = Variant.create!(
  century_id: @_2000.id,
  operator_id: @hamdi.id,
  sex: 0,
  defense: 4,
  flavour_text: "Blood sometimes gets mixed in my black coffee, I like the taste.",
  image: "operators/hamdi.jpg"
)

@v_marwa = Variant.create!(
  century_id: @_2100.id,
  operator_id: @marwa.id,
  sex: 1,
  defense: 3,
  flavour_text: "Some thought I was too pretty to give commands.",
  image: "operators/marwa.jpg"
)

@v_dr_koh = Variant.create!(
  century_id: @_2200.id,
  operator_id: @dr_koh.id,
  sex: 1,
  defense: 2,
  flavour_text: "You see yourself as a whole, in form and mind, but you are not. Let me show you.",
  image: "operators/dr_koh.jpg"
)

@v_captain_shortspear = Variant.create!(
  century_id: @_1800.id,
  operator_id: @captain_shortspear.id,
  sex: 0,
  defense: 2,
  flavour_text: "You won't pick me out in a crowd, not until my pistol is in your face.",
  image: "operators/captain_shortspear.jpg"
)

@v_soren_glasskin = Variant.create!(
  operator_id: @soren.id,
  century_id: @_1800.id,
  name: "Glasskin",
  sex: 0,
  defense: 2,
  flavour_text: "You keep calling me Soren, you really shouldn't.",
  image: "operators/soren_glasskin.jpg"
)

@v_soren_the_faceless = Variant.create!(
  operator_id: @soren.id,
  century_id: @_1900.id,
  name: "The Faceless",
  sex: 0,
  defense: 3,
  flavour_text: "Time erases all faces, but I forget none.",
  image: "operators/soren_the_faceless.jpg"
)

@v_the_gray_baron = Variant.create!(
  century_id: @_1900.id,
  operator_id: @the_gray_baron.id,
  sex: 1,
  defense: 2,
  flavour_text: "Men of power cast dark shadows. The closer a woman gets, the easier for her to hide in them.",
  image: "operators/the_gray_baron.jpg"
)

@v_dr_rosu = Variant.create!(
  century_id: @_1900.id,
  operator_id: @dr_rosu.id,
  sex: 1,
  defense: 2,
  flavour_text: "They never suspect.",
  image: "operators/dr_rosu.jpg"
)

@v_mr_rosu = Variant.create!(
  century_id: @_1900.id,
  operator_id: @mr_rosu.id,
  sex: 0,
  defense: 1,
  flavour_text: "If the devil grants you your wish, is he still the devil? Or do you become the devil by paying his price?",
  image: "operators/mr_rosu.jpg"
)

@v_zeev = Variant.create!(
  century_id: @_2100.id,
  operator_id: @zeev.id,
  sex: 2,
  defense: 4,
  flavour_text: "My arms are stronger than steel. My legs are lighter than wind. My heart burns brighter than the sun.",
  image: "operators/zeev.jpg"
)

@v_emory_iv = Variant.create!(
  century_id: @_2000.id,
  operator_id: @emory_iv.id,
  sex: 0,
  defense: 5,
  flavour_text: "What color is your bugatti, nerd?",
  image: "operators/emory_iv.jpg"
)

# Variant.create!(
#   operator_id: Operator.find@!(Striker).id,
#   century_id: @_1900.id,
#   operator_id: Operator.find@!(Striker).id,
#   sex: 2,
#   defense: 1,
#   image: "Striker.jpg"
# )

@type_human = Type.create!(
  name: "Human"
)

@type_drone = Type.create!(
  name: "Drone"
)

@type_post_human = Type.create!(
  name: "Post-Human"
)

@type_unit_707 = Type.create!(
  name: "Unit 707"
)

@type_orbital_strike_team = Type.create!(
  name: "Orbital Strike Team"
)

################################################################################
# Create Variant Types (Human/Drone/Unit 707 etc)
################################################################################

VariantType.create!(
  variant_id: @v_mori_the_piercer.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_akane.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_angela_storm.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_ben_storm.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_ben_storm_the_looper.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  type_id: @type_post_human.id
)

VariantType.create!(
  variant_id: @v_the_pale_man.id,
  type_id: @type_drone.id
)

VariantType.create!(
  variant_id: @v_the_pale_man.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_mia_star.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_mia_star.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_man_bon_hwa.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_man_bon_hwa.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_big_boy.id,
  type_id: @type_drone.id
)

VariantType.create!(
  variant_id: @v_big_boy.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_little_boy.id,
  type_id: @type_drone.id
)

VariantType.create!(
  variant_id: @v_little_boy.id,
  type_id: @type_orbital_strike_team.id
)

VariantType.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_aran.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_aran.id,
  type_id: @type_unit_707.id
)

VariantType.create!(
  variant_id: @v_layla.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_layla.id,
  type_id: @type_unit_707.id
)

VariantType.create!(
  variant_id: @v_hamdi.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_hamdi.id,
  type_id: @type_unit_707.id
)

VariantType.create!(
  variant_id: @v_marwa.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_marwa.id,
  type_id: @type_unit_707.id
)

VariantType.create!(
  variant_id: @v_dr_koh.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_captain_shortspear.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_soren_glasskin.id,
  type_id: @type_post_human.id
)

VariantType.create!(
  variant_id: @v_soren_the_faceless.id,
  type_id: @type_post_human.id
)

VariantType.create!(
  variant_id: @v_the_gray_baron.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_dr_rosu.id,
  type_id: @type_human.id
)

VariantType.create!(
  variant_id: @v_mr_rosu.id,
  type_id: @type_post_human.id
)

# VariantType.create!(
#   variant_id: @v_striker.id,
#   type_id: @type_drone.id
# )

VariantType.create!(
  variant_id: @v_zeev.id,
  type_id: @type_post_human.id
)

VariantType.create!(
  variant_id: @v_emory_iv.id,
  type_id: @type_post_human.id
)

################################################################################
# Create Agendas
################################################################################

@agenda_military = Agenda.create!(
  name: "Military"
)

@agenda_science = Agenda.create!(
  name: "Science"
)

@agenda_politics = Agenda.create!(
  name: "Politics"
)

@agenda_wild = Agenda.create!(
  name: "Wild"
)

@agenda_military_politics = Agenda.create!(
  name: "Military/Politics"
)

@agenda_military_science = Agenda.create!(
  name: "Military/Science"
)

@agenda_science_politics = Agenda.create!(
  name: "Science/Politics"
)

################################################################################
# Create Operator Abilities
# Create Operator Ability Costs (2m/3ms/2p etc)
# Create VariantAbilities (link abilities to variants in database)
################################################################################

@ability_mori1 = Ability.create!(
  text: "DISINTEGRATE a female operator with 2 defense or less",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_mori1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_mori1.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_mori_the_piercer.id,
  ability_id: @ability_mori1.id
)

@ability_mori2 = Ability.create!(
  text: "MOVE and STRIKE [3] if Mori already MOVED this turn",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_mori2.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_mori2.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_mori_the_piercer.id,
  ability_id: @ability_mori2.id
)

@ability_mori3 = Ability.create!(
  text: "After a MOVE Mori may STRIKE [1]",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_mori_the_piercer.id,
  ability_id: @ability_mori3.id
)

@ability_angela_1 = Ability.create!(
  text: "STRIKE [2]. If target is an operator you may STREAM them to the turnpoint of any operator named Ben Storm",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_angela_1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_angela_1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_angela_storm.id,
  ability_id: @ability_angela_1.id
)

@ability_angela_2 = Ability.create!(
  text: "DISABLE all enemy male operators",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_angela_2.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_angela_storm.id,
  ability_id: @ability_angela_2.id
)

@ability_ben1 = Ability.create!(
  text: "STRIKE [2]. If target is an operator you may STREAM them to the turnpoint of any operator named Angela Storm",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_ben1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_ben1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_ben_storm.id,
  ability_id: @ability_ben1.id
)

@ability_ben2 = Ability.create!(
  text: "Ambush: DISABLE any operator that MOVEs into Ben Storm's turnpoint on opponent’s next turn",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_ben2.id,
  agenda_id: @agenda_military_science.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_ben_storm.id,
  ability_id: @ability_ben2.id
)

@ability_ben_storm_the_looper1 = Ability.create!(
  text: "STRIKE [1]",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_looper1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_ben_storm_the_looper.id,
  ability_id: @ability_ben_storm_the_looper1.id
)

@ability_ben_storm_the_looper2 = Ability.create!(
  text: "STREAM to the turnpoint of target operator anywhere on the board and STRIKE [2]. At the end of the turn STREAM back to current turnpoint",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_looper2.id,
  agenda_id: @agenda_military_science.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_ben_storm_the_looper.id,
  ability_id: @ability_ben_storm_the_looper2.id
)

@ability_ben_storm_the_transcended1 = Ability.create!(
  text: "Swap current agenda for opponent’s agenda in the same century",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_transcended1.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_transcended1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  ability_id: @ability_ben_storm_the_transcended1.id
)

@ability_ben_storm_the_transcended2 = Ability.create!(
  text: "IMMOBILIZE an operator anywhere on the board",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_transcended2.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_ben_storm_the_transcended2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  ability_id: @ability_ben_storm_the_transcended2.id
)

@ability_ben_storm_the_transcended3 = Ability.create!(
  text: "When activated, DISABLE all other operators named Ben Storm and Angela Storm",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  ability_id: @ability_ben_storm_the_transcended3.id
)

@ability_akane1 = Ability.create!(
  text: "Give +2 ARMOR to another friendly operator",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_akane1.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_akane1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_akane.id,
  ability_id: @ability_akane1.id
)

@ability_akane2 = Ability.create!(
  text: "Haven: REACTIVATE a disbled operator and give them +1 ARMOR",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_akane2.id,
  agenda_id: @agenda_politics.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_akane.id,
  ability_id: @ability_akane2.id
)

@ability_akane3 = Ability.create!(
  text: "Cannot end the turn on the same turnpoint as Mori. Use an extra MOVE if Akane has already moved this turn.",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_akane.id,
  ability_id: @ability_akane3.id
)

@ability_the_pale_man1 = Ability.create!(
  text: "STRIKE [1] one century upstream or downstream",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_the_pale_man1.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_the_pale_man.id,
  ability_id: @ability_the_pale_man1.id
)

@ability_the_pale_man2 = Ability.create!(
  text: "STRIKE [3]",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_the_pale_man2.id,
  agenda_id: @agenda_military_science.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_the_pale_man.id,
  ability_id: @ability_the_pale_man2.id
)

@ability_mia_star1 = Ability.create!(
  text: "Give +1 ARMOR to all Orbital Strike Team operators on the board",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_mia_star1.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_mia_star.id,
  ability_id: @ability_mia_star1.id
)

@ability_mia_star2 = Ability.create!(
  text: "Orbital Barrage: STRIKE [2] at up to 3 different targets",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_mia_star2.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_mia_star2.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_mia_star.id,
  ability_id: @ability_mia_star2.id
)

@ability_mia_star3 = Ability.create!(
  text: "When moving, can take along a friendly operator",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_mia_star.id,
  ability_id: @ability_mia_star3.id
)

@ability_augur1 = Ability.create!(
  text: "Channel: switch Augur with any disintegrated operator. switch back if disabled. Augur returns disabled",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_augur1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  ability_id: @ability_augur1.id
)

@ability_augur2 = Ability.create!(
  text: "Stream-shade: create a copy of Augur in the opposite turnpoint. It has the type ghost, is FRAIL, and mirrors all of Augur’s actions.",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_augur2.id,
  agenda_id: @agenda_politics.id,
  quantity: 2
)

AbilityCost.create!(
  ability_id: @ability_augur2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  ability_id: @ability_augur2.id
)

@ability_man_bon_hwa1 = Ability.create!(
  text: "STRIKE [2] across stream",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_man_bon_hwa1.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_man_bon_hwa.id,
  ability_id: @ability_man_bon_hwa1.id
)

@ability_man_bon_hwa2 = Ability.create!(
  text: "DISINTEGRATE an operator activated on opponent's previous turn",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_man_bon_hwa2.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 4
)

VariantAbility.create!(
  variant_id: @v_man_bon_hwa.id,
  ability_id: @ability_man_bon_hwa2.id
)

@ability_man_bon_hwa3 = Ability.create!(
  text: "Cannot be struck by female operators",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_man_bon_hwa.id,
  ability_id: @ability_man_bon_hwa3.id
)

@ability_big_boy1 = Ability.create!(
  text: "STRIKE [2]",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_big_boy1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_big_boy1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_big_boy.id,
  ability_id: @ability_big_boy1.id
)

@ability_big_boy2 = Ability.create!(
  text: "LOCKDOWN turnpoint [no operator can MOVE in or out of the turnpoint]",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_big_boy2.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_big_boy2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_big_boy.id,
  ability_id: @ability_big_boy2.id
)

@ability_big_boy3 = Ability.create!(
  text: "Can be activated in any turnpoint but cannot MOVE on its own",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_big_boy.id,
  ability_id: @ability_big_boy3.id
)

@ability_little_boy1 = Ability.create!(
  text: "STREAM enemy operator from any turnpoint to current",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_little_boy1.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_little_boy.id,
  ability_id: @ability_little_boy1.id
)

@ability_little_boy2 = Ability.create!(
  text: "STREAM enemy operator from current to any turnpoint",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_little_boy2.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_little_boy.id,
  ability_id: @ability_little_boy2.id
)

@ability_sourayah1 = Ability.create!(
  text: "Select a turnpoint upstream. Remove all ARMOR from a random enemy operator",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_sourayah1.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_sourayah1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  ability_id: @ability_sourayah1.id
)

@ability_sourayah2 = Ability.create!(
  text: "Dark visions: IMMOBILIZE target male enemy operator in any turnpoint. He does STRIKE [3] to a random target in his turnpoint.",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_sourayah2.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_sourayah2.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  ability_id: @ability_sourayah2.id
)

@ability_sourayah3 = Ability.create!(
  text: "Cannot MOVE upstream",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  ability_id: @ability_sourayah3.id
)

@ability_aran1 = Ability.create!(
  text: "STRIKE [3]",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_aran1.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_aran.id,
  ability_id: @ability_aran1.id
)

@ability_aran2 = Ability.create!(
  text: "STRIKE [2] to an untouchable operator",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_aran2.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_aran.id,
  ability_id: @ability_aran2.id
)

@ability_aran3 = Ability.create!(
  text: "Can MOVE into any turnpoint, ignores restrictions. Cannot be immobilized",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_aran.id,
  ability_id: @ability_aran3.id
)

@ability_layla1 = Ability.create!(
  text: "REACTIVATE a Unit 707 operator",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_layla1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_layla.id,
  ability_id: @ability_layla1.id
)

@ability_layla2 = Ability.create!(
  text: "REACTIVATE an operator. If it's an enemy operator you may pay (1p) and control it for 2 turns [cannot use their tech]",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_layla2.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_layla2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_layla.id,
  ability_id: @ability_layla2.id
)

@ability_layla3 = Ability.create!(
  text: "Can MOVE 2 times in a turn",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_layla.id,
  ability_id: @ability_layla3.id
)

@ability_hamdi1 = Ability.create!(
  text: "Give +3 ARMOR to an operator from Unit 707",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_hamdi1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_hamdi1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_hamdi.id,
  ability_id: @ability_hamdi1.id
)

@ability_hamdi2 = Ability.create!(
  text: "STRIKE [3]",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_hamdi2.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

AbilityCost.create!(
  ability_id: @ability_hamdi2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_hamdi.id,
  ability_id: @ability_hamdi2.id
)

@ability_marwa1 = Ability.create!(
  text: "Recall: STREAM all friendly Unit 707 operators 707 to Marwa's turnpoint",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_marwa1.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_marwa.id,
  ability_id: @ability_marwa1.id
)

@ability_marwa2 = Ability.create!(
  text: "DISABLE all enemy male operators",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_marwa2.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_marwa.id,
  ability_id: @ability_marwa2.id
)

@ability_marwa3 = Ability.create!(
  text: "All friendly Unit 707 operators can be activated on Marwa's turnpoint",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_marwa.id,
  ability_id: @ability_marwa3.id
)

@ability_dr_koh1 = Ability.create!(
  text: "Viral corrosion: remove all ARMOR from all enemy operators in this stream",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_dr_koh1.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_dr_koh1.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_dr_koh.id,
  ability_id: @ability_dr_koh1.id
)

@ability_dr_koh2 = Ability.create!(
  text: "ACTIVATE a disintegrated friendly operator in 2200 in your stream",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_dr_koh2.id,
  agenda_id: @agenda_science.id,
  quantity: 2
)

AbilityCost.create!(
  ability_id: @ability_dr_koh2.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_dr_koh.id,
  ability_id: @ability_dr_koh2.id
)

@ability_captain_shortspear1 = Ability.create!(
  text: "MOVE. After the effects of entering the turnpoint have been applied become UNTOUCHABLE",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_captain_shortspear1.id,
  agenda_id: @agenda_military_science.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_captain_shortspear.id,
  ability_id: @ability_captain_shortspear1.id
)

@ability_captain_shortspear2 = Ability.create!(
  text: "STRIKE [2] at up to 2 different targets",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_captain_shortspear2.id,
  agenda_id: @agenda_military_science.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_captain_shortspear.id,
  ability_id: @ability_captain_shortspear2.id
)

@ability_soren_glasskin1 = Ability.create!(
  text: "Use an ability that any other operator in the same stream may use [must pay its price]",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_soren_glasskin1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_soren_glasskin.id,
  ability_id: @ability_soren_glasskin1.id
)

@ability_soren_glasskin2 = Ability.create!(
  text: "DISABLE an enemy operator. Soren becomes that operator with extra ability: pay 1x to turn back to Soren.",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_soren_glasskin2.id,
  agenda_id: @agenda_wild.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_soren_glasskin.id,
  ability_id: @ability_soren_glasskin2.id
)

@ability_soren_the_faceless1 = Ability.create!(
  text: "Chrono Parasite: Choose a target enemy operator. For 3 turns, starting with current, that operator receives STRIKE [1]",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_soren_the_faceless1.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_soren_the_faceless.id,
  ability_id: @ability_soren_the_faceless1.id
)

@ability_soren_the_faceless2 = Ability.create!(
  text: "Impair all enemy human operators",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_soren_the_faceless2.id,
  agenda_id: @agenda_wild.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_soren_the_faceless.id,
  ability_id: @ability_soren_the_faceless2.id
)

@ability_soren_the_faceless3 = Ability.create!(
  text: "At the end of each player's turn any operator that is in Soren’s turnpoint must STREAM to their original turnpoint",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_soren_the_faceless.id,
  ability_id: @ability_soren_the_faceless3.id
)

@ability_the_gray_baron1 = Ability.create!(
  text: "Spin agenda instantly without ripple effect",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_the_gray_baron1.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_the_gray_baron.id,
  ability_id: @ability_the_gray_baron1.id
)

@ability_the_gray_baron2 = Ability.create!(
  text: "STRIKE [2]",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_the_gray_baron2.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_the_gray_baron2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_the_gray_baron.id,
  ability_id: @ability_the_gray_baron2.id
)

@ability_dr_rosu1 = Ability.create!(
  text: "DISINTEGRATE an operator DISABLED on this turn",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_dr_rosu1.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

AbilityCost.create!(
  ability_id: @ability_dr_rosu1.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_dr_rosu.id,
  ability_id: @ability_dr_rosu1.id
)

@ability_dr_rosu2 = Ability.create!(
  text: "Give a friendly operator +3 ARMOR",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_dr_rosu2.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

AbilityCost.create!(
  ability_id: @ability_dr_rosu2.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_dr_rosu.id,
  ability_id: @ability_dr_rosu2.id
)

@ability_mr_rosu1 = Ability.create!(
  text: "Create a Striker in current turnpoint",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_mr_rosu1.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

VariantAbility.create!(
  variant_id: @v_mr_rosu.id,
  ability_id: @ability_mr_rosu1.id
)

@ability_mr_rosu2 = Ability.create!(
  text: "EMP Device: DISINTEGRATE all enemy drones and deployed tech",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_mr_rosu2.id,
  agenda_id: @agenda_science.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_mr_rosu.id,
  ability_id: @ability_mr_rosu2.id
)

@ability_mr_rosu3 = Ability.create!(
  text: "Can't have more than 3 Puppets of Despair on the board",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_mr_rosu.id,
  ability_id: @ability_mr_rosu3.id
)

# @ability_striker1 = Ability.create!(
#   text: "STRIKE [1]",
#   ability_type: 0
# )

# AbilityCost.create!(
#   ability_id: @ability_striker1.id,
#   agenda_id: @agenda_military.id,
#   quantity: 1
# )

# VariantAbility.create!(
#   variant_id: @v_striker.id,
#   ability_id: @ability_striker1.id
# )

# @ability_striker2 = Ability.create!(
#   text: "If Striker’s creator is DISABLED, Striker is IMMOBILIZED and IMPAIRED",
#   ability_type: 2
# )

# VariantAbility.create!(
#   variant_id: @v_striker.id,
#   ability_id: @ability_striker2.id
# )

# @ability_striker3 = Ability.create!(
#   text: "If Striker’s creator is DISINTEGRATED, Striker is also DISINTEGRATED",
#   ability_type: 3
# )

# VariantAbility.create!(
#   variant_id: @v_striker.id,
#   ability_id: @ability_striker3.id
# )

@ability_zeev1 = Ability.create!(
  text: "Mark & Destroy: DISINTEGRATE an operator with 1 defense",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_zeev1.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

VariantAbility.create!(
  variant_id: @v_zeev.id,
  ability_id: @ability_zeev1.id
)

@ability_zeev2 = Ability.create!(
  text: "STRIKE [5] at a target Controller",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_zeev2.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 5
)

VariantAbility.create!(
  variant_id: @v_zeev.id,
  ability_id: @ability_zeev2.id
)

@ability_zeev3 = Ability.create!(
  text: "When moving can take along a friendly DISABLED operator",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_zeev.id,
  ability_id: @ability_zeev3.id
)

@ability_emory_iv1 = Ability.create!(
  text: "STRIKE [1] at all enemy operators",
  ability_type: 0
)

AbilityCost.create!(
  ability_id: @ability_emory_iv1.id,
  agenda_id: @agenda_military.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_emory_iv.id,
  ability_id: @ability_emory_iv1.id
)

@ability_emory_iv2 = Ability.create!(
  text: "Make all friendly operators UNTOUCHABLE",
  ability_type: 1
)

AbilityCost.create!(
  ability_id: @ability_emory_iv2.id,
  agenda_id: @agenda_politics.id,
  quantity: 3
)

VariantAbility.create!(
  variant_id: @v_emory_iv.id,
  ability_id: @ability_emory_iv2.id
)

@ability_emory_iv3 = Ability.create!(
  text: "Once per turn, if Emory IV is being the target of an ability that was paid for using [P] he can redirect it to a new target",
  ability_type: 2
)

VariantAbility.create!(
  variant_id: @v_emory_iv.id,
  ability_id: @ability_emory_iv3.id
)

################################################################################
# Create Activation Costs
################################################################################

ActivationCostAgenda.create!(
  variant_id: @v_mori_the_piercer.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_mori_the_piercer.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_angela_storm.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_ben_storm.id,
  agenda_id: @agenda_military_science.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_ben_storm_the_looper.id,
  agenda_id: @agenda_military_science.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  agenda_id: @agenda_science.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_akane.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_akane.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_the_pale_man .id,
  agenda_id: @agenda_military_science.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_mia_star.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  agenda_id: @agenda_politics.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_man_bon_hwa .id,
  agenda_id: @agenda_military_politics.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_big_boy.id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_big_boy.id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_little_boy.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_aran.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_layla.id,
  agenda_id: @agenda_science_politics.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_hamdi.id,
  agenda_id: @agenda_military.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_marwa.id,
  agenda_id: @agenda_military_politics.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_dr_koh .id,
  agenda_id: @agenda_science.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_dr_koh .id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_captain_shortspear.id,
  agenda_id: @agenda_military_science.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_soren_glasskin.id,
  agenda_id: @agenda_wild.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_soren_the_faceless .id,
  agenda_id: @agenda_wild.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_the_gray_baron .id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_the_gray_baron .id,
  agenda_id: @agenda_wild.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_dr_rosu .id,
  agenda_id: @agenda_military.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_dr_rosu .id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_mr_rosu .id,
  agenda_id: @agenda_science.id,
  quantity: 2
)

ActivationCostAgenda.create!(
  variant_id: @v_mr_rosu .id,
  agenda_id: @agenda_wild.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_zeev.id,
  agenda_id: @agenda_military.id,
  quantity: 3
)

ActivationCostAgenda.create!(
  variant_id: @v_zeev.id,
  agenda_id: @agenda_politics.id,
  quantity: 1
)

ActivationCostAgenda.create!(
  variant_id: @v_emory_iv.id,
  agenda_id: @agenda_military.id,
  quantity: 4
)

################################################################################
# Create Tech Categories
################################################################################

@tech_category_weapon = TechCategory.create!(
  name: 0,
  icon: "icons/tech/categories/weapon.svg"
)

@tech_category_device = TechCategory.create!(
  name: 1,
  icon: "icons/tech/categories/device.svg"
)

@tech_category_relic = TechCategory.create!(
  name: 2,
  icon: "icons/tech/categories/relic.svg"
)

################################################################################
# Create Tech Types
################################################################################

@tech_type_assault = TechType.create!(
  name: 0,
  icon: "icons/tech/types/assault.svg"
)

@tech_type_heavy = TechType.create!(
  name: 1,
  icon: "icons/tech/types/heavy.svg"
)

@tech_type_remote = TechType.create!(
  name: 2,
  icon: "icons/tech/types/remote.svg"
)

@tech_type_wearable = TechType.create!(
  name: 3,
  icon: "icons/tech/types/wearable.svg"
)

@tech_type_bio = TechType.create!(
  name: 4,
  icon: "icons/tech/types/bio.svg"
)

@tech_type_ai = TechType.create!(
  name: 5,
  icon: "icons/tech/types/ai.svg"
)

@tech_type_temporal = TechType.create!(
  name: 6,
  icon: "icons/tech/types/temporal.svg"
)

@tech_type_enhancer = TechType.create!(
  name: 7,
  icon: "icons/tech/types/enhancer.svg"
)

################################################################################
# Create Variant Tech Slots (W W/W D/D R etc)
################################################################################

VariantTechSlot.create!(
  variant_id: @v_mori_the_piercer.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_mori_the_piercer.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_angela_storm.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_angela_storm.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm_the_looper.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm_the_looper.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_ben_storm_the_transcended.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_akane.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_akane.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_the_pale_man.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_the_pale_man.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_mia_star.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_mia_star.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_augur_stream_phase_shaman.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_man_bon_hwa.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_man_bon_hwa.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_big_boy.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_big_boy.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_little_boy.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_little_boy.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_sourayah_the_bad_prophet.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_aran.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_aran.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_layla.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_layla.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_hamdi.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_hamdi.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_marwa.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_marwa.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_dr_koh.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_dr_koh.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_captain_shortspear.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_captain_shortspear.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_soren_glasskin.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_soren_glasskin.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_soren_the_faceless.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_soren_the_faceless.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_the_gray_baron.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_the_gray_baron.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_dr_rosu.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_dr_rosu.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_mr_rosu.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_mr_rosu.id,
  tech_category_id: @tech_category_device.id
)

VariantTechSlot.create!(
  variant_id: @v_zeev.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_zeev.id,
  tech_category_id: @tech_category_relic.id
)

VariantTechSlot.create!(
  variant_id: @v_emory_iv.id,
  tech_category_id: @tech_category_weapon.id
)

VariantTechSlot.create!(
  variant_id: @v_emory_iv.id,
  tech_category_id: @tech_category_weapon.id
)

################################################################################
# Create Tech
# Create Tech Abilities
################################################################################

@t_electrical_trident = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "Electrical Trident",
  cost: 2,
)

@ability_t_electrical_trident = Ability.create!(
  text: "STRIKE [2]",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_electrical_trident.id,
  ability_id: @ability_t_electrical_trident.id
)

@t_the_ghost_sword_of_saturn = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "The Ghost Sword of Saturn",
  cost: 4,
)

@ability_ = Ability.create!(
  text: "EQUIP. STRIKE [3] to first enemy operator that enters turnpoint",
  ability_type: 0
)

@t_rail_gun = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "Rail Gun",
  cost: 2,
)

@ability_t_rail_gun = Ability.create!(
  text: "STRIKE [1] to 2 targets",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_rail_gun.id,
  ability_id: @ability_t_rail_gun.id
)

@t_3_ax_automated_aim_assist = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "3Ax (Automated Aim Assist)",
  cost: 3,
)

@ability_t_3_ax_automated_aim_assist = Ability.create!(
  text: "Gain +2 to next STRIKE that targets an operator",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_3_ax_automated_aim_assist.id,
  ability_id: @ability_t_3_ax_automated_aim_assist.id
)

@t_plasma_hammer = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "Plasma Hammer",
  cost: 3,
)

@ability_t_plasma_hammer = Ability.create!(
  text: "STRIKE [3]",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_plasma_hammer.id,
  ability_id: @ability_t_plasma_hammer.id
)

@t_vibroblade = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "Vibroblade",
  cost: 3,
)

@ability_t_vibroblade = Ability.create!(
  text: "STRIKE [2] at target operator. Ignore armor",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_vibroblade.id,
  ability_id: @ability_t_vibroblade.id
)

@t_dark_skies_protocol = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_assault.id,
  name: "Dark Skies Protocol",
  cost: 4,
)

@ability_t_dark_skies_protocol = Ability.create!(
  text: "STRIKE [2] to target controller anywhere on the board",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_dark_skies_protocol.id,
  ability_id: @ability_t_dark_skies_protocol.id
)

@t_dirty_chrono_bomb = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "Dirty Chrono-Bomb",
  cost: 5,
)

@ability_t_dirty_chrono_bomb = Ability.create!(
  text: "STRIKE [4] to target controller",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_dirty_chrono_bomb.id,
  ability_id: @ability_t_dirty_chrono_bomb.id
)

@t_stormpit = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "Stormpit",
  cost: 6,
)

@ability_t_stormpit = Ability.create!(
  text: "STREAM target enemy operator anywhere on the board to current turnpoint and STRIKE [2]",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_stormpit.id,
  ability_id: @ability_t_stormpit.id
)

@t_nuclear_railgun = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "Nuclear Railgun",
  cost: 4,
)

@ability_t_nuclear_railgun = Ability.create!(
  text: "STRIKE [1]. CONTAMINATE turnpoint (at the start of each player's turn operators in this turnpoint get -2 ARMOR)",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_nuclear_railgun.id,
  ability_id: @ability_t_nuclear_railgun.id
)

@t_temporal_rail_gun = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "Temporal Rail Gun",
  cost: 3,
)

@ability_t_temporal_rail_gun = Ability.create!(
  text: "STRIKE [2] across stream",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_temporal_rail_gun.id,
  ability_id: @ability_t_temporal_rail_gun.id
)

@t_plasma_mortar = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "Plasma Mortar",
  cost: 3,
)

@ability_t_plasma_mortar = Ability.create!(
  text: "DEPLOY.  At the start of your turn STRIKE [2] to controllers for 3 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_plasma_mortar.id,
  ability_id: @ability_t_plasma_mortar.id
)

@t_gml_facelock = Tech.create!(
  tech_category_id: @tech_category_weapon.id,
  tech_type_id: @tech_type_heavy.id,
  name: "GML Facelock",
  cost: 3,
)

@ability_t_gml_facelock = Ability.create!(
  text: "DEPLOY. Choose a target enemy operator on the board. At the start of your next turn GML Facelock does STRIKE [3] to that operator",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_gml_facelock.id,
  ability_id: @ability_t_gml_facelock.id
)

@t_emp_bomb = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_assault.id,
  name: "EMP Bomb",
  cost: 5
)

@ability_t_emp_bomb = Ability.create!(
  text: "DISINTEGRATE all enemy tech",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_emp_bomb.id,
  ability_id: @ability_t_emp_bomb.id
)

@t_corrosive_nanobot_swarm = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_assault.id,
  name: "Corrosive Nanobot Swarm",
  cost: 4
)

@ability_t_corrosive_nanobot_swarm = Ability.create!(
  text: "Remove all ARMOR from an enemy operator",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_corrosive_nanobot_swarm.id,
  ability_id: @ability_t_corrosive_nanobot_swarm.id
)

@t_viral_nanobot_swarm = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_remote.id,
  name: "Viral Nanobot Swarm",
  cost: 3
)

@ability_t_viral_nanobot_swarm = Ability.create!(
  text: "DEPLOY. At the start of each player's turn Viral Nanobot Swarm moves 1 century downstream and all enemy operators in the turnpoint get -2 ARMOR",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_viral_nanobot_swarm.id,
  ability_id: @ability_t_viral_nanobot_swarm.id
)

@t_emp_nanobot_swarm = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_remote.id,
  name: "EMP Nanobot Swarm",
  cost: 3
)

@ability_t_emp_nanobot_swarm = Ability.create!(
  text: "DEPLOY. At the start of each player's turn EMP Nanobot Swarm moves 1 century downstream and DISINTEGRATES all enemy tech",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_emp_nanobot_swarm.id,
  ability_id: @ability_t_emp_nanobot_swarm.id
)

@t_m_u_g_g_e_r_droid = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_remote.id,
  name: "M.U.G.G.E.R. Droid",
  cost: 3
)

@ability_t_m_u_g_g_e_r_droid = Ability.create!(
  text: "DEPLOY. Can MOVE like an operator. Can take tech from operators and give tech to operators [tech restrictions apply. Can hold 1 tech at a time]",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_m_u_g_g_e_r_droid.id,
  ability_id: @ability_t_m_u_g_g_e_r_droid.id
)

@t_pulse_shield = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_wearable.id,
  name: "Pulse Shield",
  cost: 2
)

@ability_t_pulse_shield = Ability.create!(
  text: "EQUIP. After owner receives STRIKE damage gain +2 ARMOR",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_pulse_shield.id,
  ability_id: @ability_t_pulse_shield.id
)

@t_stealth_suit = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_wearable.id,
  name: "Stealth Suit",
  cost: 3
)

@ability_t_stealth_suit = Ability.create!(
  text: "Target operator becomes UNTOUCHABLE for 2 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_stealth_suit.id,
  ability_id: @ability_t_stealth_suit.id
)

@t_exo_suit = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_wearable.id,
  name: "Exo-Suit",
  cost: 4
)

@ability_t_exo_suit = Ability.create!(
  text: "Gain +4 ARMOR",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_exo_suit.id,
  ability_id: @ability_t_exo_suit.id
)

@t_neural_parasite = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Neural Parasite",
  cost: 5
)

@ability_t_neural_parasite = Ability.create!(
  text: "Take control of target enemy operator for 2 turns (cannot use their tech)",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_neural_parasite.id,
  ability_id: @ability_t_neural_parasite.id
)

@t_stim_pack = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Stim-Pack",
  cost: 2
)

@ability_t_stim_pack = Ability.create!(
  text: "Give friendly operator +2 to their next STRIKE",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_stim_pack.id,
  ability_id: @ability_t_stim_pack.id
)

@t_b_m_b_l_black_market_biohacking_lab = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "B.M.B.L. (Black Market Biohacking Lab)",
  cost: 3
)

@ability_t_b_m_b_l_black_market_biohacking_lab = Ability.create!(
  text: "DEPLOY. Friendly operators may permanently change their type/sex",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_b_m_b_l_black_market_biohacking_lab.id,
  ability_id: @ability_t_b_m_b_l_black_market_biohacking_lab.id
)

@t_neuro_tuning_chip = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Neuro-Tuning Chip",
  cost: 2
)

@ability_t_neuro_tuning_chip = Ability.create!(
  text: "Gain +2 ARMOR",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_neuro_tuning_chip.id,
  ability_id: @ability_t_neuro_tuning_chip.id
)

@t_cronenberg_chamber = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Cronenberg Chamber",
  cost: 3
)

@ability_t_cronenberg_chamber = Ability.create!(
  text: "Choose one: change the sex of an operator or change one of the types of an operator",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_cronenberg_chamber.id,
  ability_id: @ability_t_cronenberg_chamber.id
)

@t_life_pod = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Life Pod",
  cost: 3
)

@ability_t_life_pod = Ability.create!(
  text: "REACTIVATE a disabled friendly operator",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_life_pod.id,
  ability_id: @ability_t_life_pod.id
)

@t_gene_adaptable_perfume = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_bio.id,
  name: "Gene-adaptable perfume",
  cost: 4
)

@ability_t_gene_adaptable_perfume = Ability.create!(
  text: "Choose an enemy operator’s ability. They must use it on another enemy operator without paying the agenda cost",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_gene_adaptable_perfume.id,
  ability_id: @ability_t_gene_adaptable_perfume.id
)

@t_spin_doctor_ai = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_ai.id,
  name: "Spin Doctor AI",
  cost: 3
)

@ability_t_spin_doctor_ai = Ability.create!(
  text: "Gain +3 ARMOR",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_spin_doctor_ai.id,
  ability_id: @ability_t_spin_doctor_ai.id
)

@t_fake_news_generator = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_ai.id,
  name: "Fake News Generator",
  cost: 1
)

@ability_t_fake_news_generator = Ability.create!(
  text: "Gain +2 to next STRIKE",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_fake_news_generator.id,
  ability_id: @ability_t_fake_news_generator.id
)

@t_automated_damage_control = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_ai.id,
  name: "Automated Damage Control",
  cost: 4
)

@ability_t_automated_damage_control = Ability.create!(
  text: "EQUIP to a controller. Whenever that controller receives STRIKE damage reduce it by 1",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_automated_damage_control.id,
  ability_id: @ability_t_automated_damage_control.id
)

@t_reflective_field_barrier = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_ai.id,
  name: "Reflective Field Barrier",
  cost: 5
)

@ability_t_reflective_field_barrier = Ability.create!(
  text: "Make controller UNTOUCHABLE for 2 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_reflective_field_barrier.id,
  ability_id: @ability_t_reflective_field_barrier.id
)

@t_dark_skies_network = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_ai.id,
  name: "Dark Skies Network",
  cost: 4
)

@ability_t_dark_skies_network = Ability.create!(
  text: "DEPLOY. Target enemy controller anywhere on board. If it moves it receives STRIKE [3]",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_dark_skies_network.id,
  ability_id: @ability_t_dark_skies_network.id
)

@t_faraday_cage = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Faraday Cage",
  cost: 3
)

@ability_t_faraday_cage = Ability.create!(
  text: "IMMOBILIZE enemy controller for 2 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_faraday_cage.id,
  ability_id: @ability_t_faraday_cage.id
)

@t_sat_p_short_range_ai_temporal_prediction_system = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "SAT-P (Short-range AI Temporal Prediction system)",
  cost: 4
)

@ability_t_sat_p_short_range_ai_temporal_prediction_system = Ability.create!(
  text: "Give +3 ARMOR to a controller.",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_sat_p_short_range_ai_temporal_prediction_system.id,
  ability_id: @ability_t_sat_p_short_range_ai_temporal_prediction_system.id
)

@t_temporal_tripwire = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Temporal Tripwire",
  cost: 3
)

@ability_t_temporal_tripwire = Ability.create!(
  text: "DEPLOY. STRIKE [2] to first enemy operator that moves into the turnpoint",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_temporal_tripwire.id,
  ability_id: @ability_t_temporal_tripwire.id
)

@t_plunger = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Plunger",
  cost: 3
)

@ability_t_plunger = Ability.create!(
  text: "STREAM an enemy operator from any turnpoint into current turnpoint",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_plunger.id,
  ability_id: @ability_t_plunger.id
)

@t_temporal_reflector = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Temporal Reflector",
  cost: 2
)

@ability_t_temporal_reflector = Ability.create!(
  text: "DEPLOY. LOCKDOWN turnpoint for 3 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_temporal_reflector.id,
  ability_id: @ability_t_temporal_reflector.id
)

@t_temporal_scrambler = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Temporal Scrambler",
  cost: 1
)

@ability_t_temporal_scrambler = Ability.create!(
  text: "Spin agenda without ripple effect",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_temporal_scrambler.id,
  ability_id: @ability_t_temporal_scrambler.id
)

@t_c_d_u_chrono_diffuser_unit = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "C.D.U. (Chrono Diffuser Unit)",
  cost: 4
)

@ability_t_c_d_u_chrono_diffuser_unit = Ability.create!(
  text: "IMMOBILIZE all enemy operators for 2 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_c_d_u_chrono_diffuser_unit.id,
  ability_id: @ability_t_c_d_u_chrono_diffuser_unit.id
)

@t_unstable_wormhole_generator = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Unstable Wormhole Generator",
  cost: 2
)

@ability_t_unstable_wormhole_generator = Ability.create!(
  text: "Turn agenda to any type",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_unstable_wormhole_generator.id,
  ability_id: @ability_t_unstable_wormhole_generator.id
)

@t_mars_rising = Tech.create!(
  tech_category_id: @tech_category_device.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Mars Rising",
  cost: 3
)

@ability_t_mars_rising = Ability.create!(
  text: "STRIKE [1] to any target anywhere on the board",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_mars_rising.id,
  ability_id: @ability_t_mars_rising.id
)

@t_void_frequency_hammer = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_assault.id,
  name: "Void-Frequency Hammer",
  cost: 2
)

@ability_t_void_frequency_hammer = Ability.create!(
  text: "Next STRIKE that targets an operator gains +2. If target operator has any equipped tech, target one and DISINTEGRATE it",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_void_frequency_hammer.id,
  ability_id: @ability_t_void_frequency_hammer.id
)

@t_boneboil_staff = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_assault.id,
  name: "Boneboil Staff",
  cost: 4
)

@ability_t_boneboil_staff = Ability.create!(
  text: "DEPLOY. At the start of your turn STRIKE [1] to all enemy operators in turnpoint for 2 turns",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_boneboil_staff.id,
  ability_id: @ability_t_boneboil_staff.id
)

@t_smoke_water = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_assault.id,
  name: "Smoke Water",
  cost: 3
)

@ability_t_smoke_water = Ability.create!(
  text: "Choose an enemy operator’s ability. They cannot use it for 3 turns.",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_smoke_water.id,
  ability_id: @ability_t_smoke_water.id
)

@t_mirror_of_ishtar = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_enhancer.id,
  name: "Mirror of Ishtar",
  cost: 4
)

@ability_t_mirror_of_ishtar = Ability.create!(
  text: "Choose a target operator’s ability. Use it once without paying the agenda cost.",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_mirror_of_ishtar.id,
  ability_id: @ability_t_mirror_of_ishtar.id
)

@t_lunar_phase_reflector = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_enhancer.id,
  name: "Lunar Phase Reflector",
  cost: 3
)

@ability_t_lunar_phase_reflector = Ability.create!(
  text: "When owner uses a STRIKE ability, duplicate the first STRIKE to another target",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_lunar_phase_reflector.id,
  ability_id: @ability_t_lunar_phase_reflector.id
)

@t_chrono_accelerator = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_enhancer.id,
  name: "Chrono-Accelerator",
  cost: 3
)

@ability_t_chrono_accelerator = Ability.create!(
  text: "MOVE two extra times this turn",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_chrono_accelerator.id,
  ability_id: @ability_t_chrono_accelerator.id
)

@t_hamsa_necklace = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_enhancer.id,
  name: "Hamsa Necklace",
  cost: 4
)

@ability_t_hamsa_necklace = Ability.create!(
  text: "EQUIP. Operator has +1 defense whilst Hamsa Necklace is equipped",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_hamsa_necklace.id,
  ability_id: @ability_t_hamsa_necklace.id
)

@t_zaman_s_vortex = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Zaman's Vortex",
  cost: 4
)

@ability_t_zaman_s_vortex = Ability.create!(
  text: "Undo last 3 move actions, including ones in opponent's turn. Any other actions remain in effect",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_zaman_s_vortex.id,
  ability_id: @ability_t_zaman_s_vortex.id
)

@t_soul_capsule = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Soul Capsule",
  cost: 5
)

@ability_t_soul_capsule = Ability.create!(
  text: "EQUIP. If the operator would be disintegrated they instead return to the same turnpoint. They have the type ghost and all other types are removed. They are FRAIL",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_soul_capsule.id,
  ability_id: @ability_t_soul_capsule.id
)

@t_the_scroll_of_zaman = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "The Scroll of Zaman",
  cost: 2
)

@ability_t_the_scroll_of_zaman = Ability.create!(
  text: "Undo opponent's agenda spin from last turn",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_the_scroll_of_zaman.id,
  ability_id: @ability_t_the_scroll_of_zaman.id
)

@t_parasitic_ghost = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Parasitic Ghost",
  cost: 5
)

@ability_t_parasitic_ghost = Ability.create!(
  text: "Create a copy of this operator. It is FRAIL and has 1 defense",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_parasitic_ghost.id,
  ability_id: @ability_t_parasitic_ghost.id
)

@t_hand_of_zaman = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Hand of Zaman",
  cost: 4
)

@ability_t_hand_of_zaman = Ability.create!(
  text: "DEPLOY. Operator may use their abilities in Hand of Zaman’s turnpoint, regardless of which turnpoint they are in",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_hand_of_zaman.id,
  ability_id: @ability_t_hand_of_zaman.id
)

@t_zeitgeist_capsule = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Zeitgeist Capsule",
  cost: 6
)

@ability_t_zeitgeist_capsule = Ability.create!(
  text: "Activate target enemy agenda as your own. It remains activated on your opponent’s next turn.",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_zeitgeist_capsule.id,
  ability_id: @ability_t_zeitgeist_capsule.id
)

@t_the_sun_mouth = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "The Sun Mouth",
  cost: 3
)

@ability_t_the_sun_mouth = Ability.create!(
  text: "DEPLOY. Activate as [political] agenda, then DISINTEGRATE",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_the_sun_mouth.id,
  ability_id: @ability_t_the_sun_mouth.id
)

@t_pyramid_blueprints = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Pyramid Blueprints",
  cost: 3
)

@ability_t_pyramid_blueprints = Ability.create!(
  text: "DEPLOY. Activate as [science] agenda, then DISINTEGRATE",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_pyramid_blueprints.id,
  ability_id: @ability_t_pyramid_blueprints.id
)

@t_war_shrine = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "War Shrine",
  cost: 3
)

@ability_t_war_shrine = Ability.create!(
  text: "DEPLOY. Activate as [military] agenda, then DISINTEGRATE",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_war_shrine.id,
  ability_id: @ability_t_war_shrine.id
)

@t_records_of_ishtar = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Records of Ishtar",
  cost: 3
)

@ability_t_records_of_ishtar = Ability.create!(
  text: "STREAM all operators to their original turnpoint",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_records_of_ishtar.id,
  ability_id: @ability_t_records_of_ishtar.id
)

@t_the_maze_of_moonlight = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "The Maze of Moonlight",
  cost: 1
)

@ability_t_the_maze_of_moonlight = Ability.create!(
  text: "EQUIP. If owner was a target of STRIKE ability on opponent's turn, use the same STRIKE ability on any target without paying the agenda price",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_the_maze_of_moonlight.id,
  ability_id: @ability_t_the_maze_of_moonlight.id
)

@t_auroral_charge = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Auroral Charge",
  cost: 0
)

@ability_t_auroral_charge = Ability.create!(
  text: "Add 3 Vremenium charge to target controller",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_auroral_charge.id,
  ability_id: @ability_t_auroral_charge.id
)

@t_lightning_stick = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Lightning Stick",
  cost: 2
)

@ability_t_lightning_stick = Ability.create!(
  text: "Deplete 3 Vremenium charge from target controller",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_lightning_stick.id,
  ability_id: @ability_t_lightning_stick.id
)

@t_well_of_stillness = Tech.create!(
  tech_category_id: @tech_category_relic.id,
  tech_type_id: @tech_type_temporal.id,
  name: "Well of Stillness",
  cost: 2
)

@ability_t_well_of_stillness = Ability.create!(
  text: "STREAM to any turnpoint with the same agenda type",
  ability_type: 0
)

TechAbility.create!(
  tech_id: @t_well_of_stillness.id,
  ability_id: @ability_t_well_of_stillness.id
)
