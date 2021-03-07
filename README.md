# dotfiles
Our dotfiles for various tools we use on stream.

## Quick installation

We use Tmux Plugin Manager, follow the installation instructions
[here](https://github.com/tmux-plugins/tpm#installation).

Checkout this repo, and from repo directory run

```bash
ln -s ${PWD}/dot.tmux.conf ~/.tmux.conf
```

Then press C-a I to install the plugins.

## How do you share your tmux on stream?

First of all, we share a Linux box we both ssh into as the same user.

The trick is to create a tmux client targeting an existing tmux session. Here
are the aliases we set up in our `.zshrc` (or `.bashrc`):

```bash
function newtmux() {
  tmux new-session -d -t stuck
}

function fiskentmux() {
  tmux attach -t fisken || (newtmux && tmux new-session -t stuck -s fisken)
}

function satutmux() {
  tmux attach -t satu || (newtmux && tmux new-session -t stuck -s satu)
}
```

So each of us only has to run either `fiskentmux` or `satutmux`.
