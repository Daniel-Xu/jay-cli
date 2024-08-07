# jay-cli （only for learning purpose)
Listen music via CLI

* Currently, it's only tested on MacOS

 ```
          _____                    _____                _____                            _____                    _____            _____          
         /\    \                  /\    \              |\    \                          /\    \                  /\    \          /\    \         
        /::\    \                /::\    \             |:\____\                        /::\    \                /::\____\        /::\    \        
        \:::\    \              /::::\    \            |::|   |                       /::::\    \              /:::/    /        \:::\    \       
         \:::\    \            /::::::\    \           |::|   |                      /::::::\    \            /:::/    /          \:::\    \      
          \:::\    \          /:::/\:::\    \          |::|   |                     /:::/\:::\    \          /:::/    /            \:::\    \     
           \:::\    \        /:::/__\:::\    \         |::|   |                    /:::/  \:::\    \        /:::/    /              \:::\    \    
           /::::\    \      /::::\   \:::\    \        |::|   |                   /:::/    \:::\    \      /:::/    /               /::::\    \   
  _____   /::::::\    \    /::::::\   \:::\    \       |::|___|______            /:::/    / \:::\    \    /:::/    /       ____    /::::::\    \  
 /\    \ /:::/\:::\    \  /:::/\:::\   \:::\    \      /::::::::\    \          /:::/    /   \:::\    \  /:::/    /       /\   \  /:::/\:::\    \ 
/::\    /:::/  \:::\____\/:::/  \:::\   \:::\____\    /::::::::::\    \        /:::/____/     \:::\____\/:::/____/       /::\   \/:::/  \:::\____\
\:::\  /:::/    \::/    /\::/    \:::\  /:::/    /   /:::/~~~~/~~ \ ___\       \:::\    \      \::/    /\:::\    \       \:::\  /:::/    \::/    /
 \:::\/:::/    / \/____/  \/____/ \:::\/:::/    /   /:::/    /                  \:::\    \      \/____/  \:::\    \       \:::\/:::/    / \/____/ 
  \::::::/    /                    \::::::/    /   /:::/    /                    \:::\    \               \:::\    \       \::::::/    /          
   \::::/    /                      \::::/    /   /:::/    /                      \:::\    \               \:::\    \       \::::/____/           
    \::/    /                       /:::/    /    \::/    /                        \:::\    \               \:::\    \       \:::\    \           
     \/____/                       /:::/    /      \/____/                          \:::\    \               \:::\    \       \:::\    \          
                                  /:::/    /                                         \:::\    \               \:::\    \       \:::\    \         
                                 /:::/    /                                           \:::\____\               \:::\____\       \:::\____\        
                                 \::/    /                                             \::/    /                \::/    /        \::/    /        
                                  \/____/                                               \/____/                  \/____/          \/____/         


```

![2023-08-01 at 12 51 AM](https://github.com/Daniel-Xu/jay-cli/assets/548144/6b7ee34d-63c4-49bc-b705-778ad6355887)

## How to install
```
cargo install jay-cli
```

## How to use
There are two commands: `single` and `random`
* single: you need to choose song manually
* random: song is choson for you randomly
* loop-one: choose one song and loop forever
```
Usage: jay-cli <COMMAND>

Commands:
  single  Choose song manually one by one after playing
  random  Song automatically chosen for you after playing
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Architecture

Might be a little bit complex than it should be!

![image](https://github.com/Daniel-Xu/jay-cli/assets/548144/a9224ba7-4a0c-4e69-9a38-2657aeb53aea)

