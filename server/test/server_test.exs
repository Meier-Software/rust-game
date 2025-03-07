defmodule ServerTest do
  use ExUnit.Case
  doctest Server

  test "server line to command" do
    case Client.Commands.line_to_commands("sample abc") do
      [a, b] ->
        assert a == "sample"
        assert b == "abc"
    end
  end
end
