defmodule Rpg.Entity do
  @moduledoc """
  Represents an entity in the RPG game mode.
  This module handles the lifecycle and behavior of game entities.
  """

  @doc """
  Starts a new entity process.

  Returns a process that represents the entity with an empty info map.
  """
  def start() do
    info = %{}
    loop_entity(info)
  end

  @doc """
  Main entity loop that handles incoming messages.

  ## Parameters
    * info - Map containing the entity's state information

  ## Messages
    * {:die, pid} - Terminates the process identified by pid
    * :stop - Stops the entity process normally
  """
  defp loop_entity(info) do
    receive do
      {:die, pid} ->
        Process.exit(pid, :die)
        loop_entity(info)
      :stop ->
        :ok
    end
  end

  @doc """
  Stops the entity process normally.
  """
  def stop(pid) do
    send(pid, :stop)
  end
end
