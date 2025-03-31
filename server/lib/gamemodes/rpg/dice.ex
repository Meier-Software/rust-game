defmodule RPG.Dice do


  def roll(dice_notation) do
    # Example dice notation: "2d6+3" means roll 2 six-sided dice and add 3
    [count, rest] = String.split(dice_notation, "d")
    {sides, modifier} = case String.split(rest, "+") do
      [sides, mod] -> {sides, String.to_integer(mod)}
      [sides] -> {sides, 0}
    end

    count = String.to_integer(count)
    sides = String.to_integer(sides)

    rolls = for _i <- 1..count do
      :rand.uniform(sides)
    end

    Enum.sum(rolls) + modifier
  end
end
