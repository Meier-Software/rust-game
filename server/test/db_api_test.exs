defmodule DbApiTest do
  require Logger
  use ExUnit.Case
  doctest Server

  test "set player" do
    a = Db.DbApi.new_player("test-guy-10", "123")
    case a do
      {:player, false} -> Logger.error "DB Test fail."
      {:player, player}  -> Logger.info "WOrks."
    end
  end

  test "get player" do
    {:player, _player} = Db.DbApi.login("test-guy-10", "123")
  end

  test "remove player" do
    {:player, :deleted} = Db.DbApi.delete_user("test-guy-10")
  end
end
