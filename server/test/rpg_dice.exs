defmodule RPG.DiceTest do
  use ExUnit.Case
  doctest RPG.Dice

  test "basic dice roll is within bounds" do
    result = RPG.Dice.roll("1d6")
    assert result >= 1 and result <= 6
  end

  test "multiple dice rolls add up correctly" do
    result = RPG.Dice.roll("2d6")
    assert result >= 2 and result <= 12
  end

  test "dice roll with modifier" do
    result = RPG.Dice.roll("1d6+3")
    assert result >= 4 and result <= 9
  end

  test "complex dice roll" do
    result = RPG.Dice.roll("3d8+5")
    assert result >= 8 and result <= 29
  end

  test "dice notation parsing" do
    # Test that invalid notation raises error
    assert_raise MatchError, fn ->
      RPG.Dice.roll("invalid")
    end
  end
end
