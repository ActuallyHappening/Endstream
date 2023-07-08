class CardsController < ApplicationController
  def index
    @variants = Variant.includes(:operator).all
    @techs = Tech.includes(:tech_category).includes(tech_abilities: :ability).all
  end

  def json
    variants = Variant.all
                      .includes(:operator)
                      .includes(variant_tech_slots: :tech_category)
                      .includes(:century)
                      .includes(variant_abilities: [ability: [ability_costs: :agenda]])

                      cards = get_card_data(variants)

    render json: cards
  end

  private

  def get_tech_slots(variant)
    tech_slots = []

    variant.variant_tech_slots.each do |variant_tech_slot|
      tech_slots << variant_tech_slot.tech_category.name
    end

    tech_slots
  end

  def get_types(variant)
    types = []

    variant.variant_types.each do |variant_type|
      types << variant_type.type.name
    end

    types
  end

  def get_ability_costs(ability)
    ability_costs = []

    ability.ability_costs.each do |ability_cost|
      ability_costs << { agenda: ability_cost.agenda.name, quantity: ability_cost.quantity }
    end

    ability_costs
  end

  def get_abilities(variant)
    abilities = []
    ability = {}

    variant.variant_abilities.each do |variant_ability|
      ability[:text] = variant_ability.ability.text
      ability[:type] = variant_ability.ability.ability_type
      ability[:cost] = get_ability_costs(variant_ability.ability)

      abilities << ability
    end

    abilities
  end

  def get_card_data(variants)
    cards = {
      controllers: [],
      operators: []
    }

    variants.each do |variant|
      card = {
        operator_name: variant.operator.name,

        century: variant.century.name,

        variant_name: variant.name,
        defense: variant.defense,
        sex: variant.sex,
        flavour_text: variant.flavour_text,
        image: variant.image,

        tech_slots: get_tech_slots(variant),
        types: get_types(variant),
        abilities: get_abilities(variant)
      }

      cards[:operators] << card
    end

    cards
  end
end
