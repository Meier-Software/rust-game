defmodule Db.DbApi do
  @moduledoc """
  A syncronous api for accessing the database
  """

  def get_player(username) do
    send(:database, {:get, :player, username, self()})

    receive do
      {:player, value} -> {:player, value}
    end
  end

  def new_player(username, password) do
    send(:database, {:new, :player, username, password, self()})

    receive do
      {:player, value} -> {:player, value}
    end
  end
end
