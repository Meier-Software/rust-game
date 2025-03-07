defmodule Client.Commands do
  def line_to_commands(line) do
    [line | _rest] = String.split(line, "\r\n")
    String.split(line, " ", trim: true)
  end

  # TODO: Turn the code from authed and client into some sort of short hand macro or something
  # To help with the help command.
  # maybe a map that contains command names and such and then runs a function with
  # that many args and if it can't find it idk disconnect them lol
  # I'd have to pass in like a lot of info oof
end
