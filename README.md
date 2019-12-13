# hearthstone-to-markdown

This is a command line tool to generate a formatted markdown table when given a HearthStone deck code.

## Usage
`hearthstone-to-markdown AAECAf0EBs/vAqCAA6iHA6+HA5aaA4qeAwxNigHJA+wHquwCt/ECw/gCpocDzYkDlZQD55UDoJsDAA==`

```
| Mana | Card Name                                                    | Qty  |                            Links                             |
| :--: | :----------------------------------------------------------- | :--: | :----------------------------------------------------------: |
|  1  | [Daring Fire-Eater](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/TRL_390.png) |  2  | [HSReplay](https://hsreplay.net/cards/50381/),[Wiki](https://hearthstone.gamepedia.com/Daring%20Fire%2DEater) |
|  2  | [Doomsayer](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/NEW1_021.png) |  2  | [HSReplay](https://hsreplay.net/cards/138/),[Wiki](https://hearthstone.gamepedia.com/Doomsayer) |
|  2  | [Khadgar](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/DAL_575.png) |  1  | [HSReplay](https://hsreplay.net/cards/52502/),[Wiki](https://hearthstone.gamepedia.com/Khadgar) |
|  3  | [Messenger Raven](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/DAL_163.png) |  2  | [HSReplay](https://hsreplay.net/cards/51943/),[Wiki](https://hearthstone.gamepedia.com/Messenger%20Raven) |
|  3  | [Phantom Militia](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/GIL_207.png) |  2  | [HSReplay](https://hsreplay.net/cards/46634/),[Wiki](https://hearthstone.gamepedia.com/Phantom%20Militia) |
|  3  | [Voodoo Doll](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/GIL_614.png) |  2  | [HSReplay](https://hsreplay.net/cards/47287/),[Wiki](https://hearthstone.gamepedia.com/Voodoo%20Doll) |
|  4  | [Hecklebot](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/DAL_058.png) |  2  | [HSReplay](https://hsreplay.net/cards/51733/),[Wiki](https://hearthstone.gamepedia.com/Hecklebot) |
|  4  | [Polymorph](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/CS2_022.png) |  2  | [HSReplay](https://hsreplay.net/cards/77/),[Wiki](https://hearthstone.gamepedia.com/Polymorph) |
|  5  | [Blast Wave](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/TRL_317.png) |  2  | [HSReplay](https://hsreplay.net/cards/50086/),[Wiki](https://hearthstone.gamepedia.com/Blast%20Wave) |
|  5  | [Zilliax](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/BOT_548.png) |  1  | [HSReplay](https://hsreplay.net/cards/49184/),[Wiki](https://hearthstone.gamepedia.com/Zilliax) |
|  6  | [Blizzard](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/CS2_028.png) |  2  | [HSReplay](https://hsreplay.net/cards/457/),[Wiki](https://hearthstone.gamepedia.com/Blizzard) |
|  6  | [Toki, Time-Tinker](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/GIL_549.png) |  1  | [HSReplay](https://hsreplay.net/cards/47055/),[Wiki](https://hearthstone.gamepedia.com/Toki%2C%20Time%2DTinker) |
|  7  | [Astromancer](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/BOT_256.png) |  2  | [HSReplay](https://hsreplay.net/cards/48195/),[Wiki](https://hearthstone.gamepedia.com/Astromancer) |
|  7  | [Flamestrike](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/CS2_032.png) |  2  | [HSReplay](https://hsreplay.net/cards/1004/),[Wiki](https://hearthstone.gamepedia.com/Flamestrike) |
|  7  | [Jan'alai, the Dragonhawk](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/TRL_316.png) |  1  | [HSReplay](https://hsreplay.net/cards/50088/),[Wiki](https://hearthstone.gamepedia.com/Jan%27alai%2C%20the%20Dragonhawk) |
|  8  | [Hex Lord Malacrass](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/TRL_318.png) |  1  | [HSReplay](https://hsreplay.net/cards/50095/),[Wiki](https://hearthstone.gamepedia.com/Hex%20Lord%20Malacrass) |
|  8  | [Power of Creation](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/DAL_578.png) |  2  | [HSReplay](https://hsreplay.net/cards/52640/),[Wiki](https://hearthstone.gamepedia.com/Power%20of%20Creation) |
|  10  | [Kalecgos](https://art.hearthstonejson.com/v1/render/latest/enUS/512x/DAL_609.png) |  1  | [HSReplay](https://hsreplay.net/cards/53002/),[Wiki](https://hearthstone.gamepedia.com/Kalecgos) |

**Total Dust:**: 14360
**Deck Code:** `AAECAf0EBs/vAqCAA6iHA6+HA5aaA4qeAwxNigHJA+wHquwCt/ECw/gCpocDzYkDlZQD55UDoJsDAA==`
```

This program uses the [hearthstonejson.com API](https://hearthstonejson.com/).