defmodule Db.DbApi do
  @moduledoc """
  A syncronous api for accessing the database
  """

  def get_player(username) do
    # Cap the username at 16 characters
    if String.length(username) <= 16 do
      send(:database, {:get, :player, username, self()})

      receive do
        {:player, value} -> {:player, value}
      end
    else
      {:player, false}
    end
  end

  def new_player(username, password) do
    if String.length(username) <= 16 do
      send(:database, {:new, :player, username, password, self()})

      receive do
        {:player, value} -> {:player, value}
      end
    end
  end

  def login(username, password) do
    send(:database, {:login, :player, username, password, self()})

    receive do
      {:player, value} -> {:player, value}
    end
  end
end
