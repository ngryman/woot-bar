config='
# `woot-bar` updates every 4 seconds
set -g status-interval 4

# display minimalist stats via woot-bar
set -g status-right "#(/usr/local/bin/tmux-status-bar) "
'

try_install_in() {
  local config_file=$1

  if [[ -f $config_file ]]; then
    echo "$config" >> $config_file
    printf "\e[32m🚀 Configured tmux successfully\e[0m\n"

    printf "👀 Here are the last 10 lines of your config file:"
    printf "\e[2m\n\n"
    printf "   │ ...\n"
    printf "   │\n"
    tail -10 $config_file | sed 's/^/   │ /'
    printf "\e[0m\n"

    printf "🏄 Done! Restart tmux and enjoy.\n"
  else
    return 1
  fi
}

main() {
  curl -sL -o /usr/local/bin/woot-bar https://github.com/ngryman/woot-bar/releases/download/0.1.0/woot-bar &&
  chmod +x /usr/local/bin/woot-bar
  printf "\e[32m😎 Installed successfully!\e[0m\n"

  printf "❓ Do you want to automatically configure tmux? \e[36m(Y/n)\e[0m "
  read -n1 configure_tmux

  if [[ $configure_tmux = "" ]]; then
    configure_tmux="Y"
  else
    printf "\n"
  fi

  if [[ $configure_tmux = "Y" ]]; then
    try_install_in ~/.tmux.conf ||
    try_install_in ~/.config/tmux/tmux.conf ||
    printf "\e[31mUnable to find your tmux config. Please create one and restart the script.\e[0m\n"
  fi
}

main
