defmodule Tag do
  @moduledoc """
  This is a tag style minigame.
  """
  def start() do
    Logger.info("Tag minigame running in zone minigames@games.ablecorp.us")
  end

  def is_player_it?(player_pid) do
    {true}
  end
end
