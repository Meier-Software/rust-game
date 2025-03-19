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

  test "player move test" do
    # Arguably this is property based testing.
    username = "test-guy-" <> inspect(:rand.uniform(10))

    {:ok, player_id} = Task.start(fn -> Player.start(self(), username) end)
    send(player_id, {:move, 10, 10, self()})

    # TODO: Recieve the event sent back.
  end
end
