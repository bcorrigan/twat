# Twitter and Telegram bot

This is a simple alerting bot that will apply rules to filter messages on twitter and relay to an individual or channel.

Examples:

- If the railway company twitter account mentions your station or line between 6AM and 8AM, receive it on telegram, else ignore.
- If the police twitter account mentions your neighbourhood
- If the bus company mention the service you get
- If the health board tweets covid numbers
- If the bookies tweet a special offer
- If your football club tweets match updates

etc etc...

# Example Config File

```
#![enable(implicit_some)]
(
    telegram: (
        bot_token: "TELEGRAM_BOT_TOKEN_HERE"
    ),
    twitter: (
        key: "TWITTER_KEY_HERE",
        secret: "TWITTER_SECRET_HERE",
    ),
    rules: [
        ( 
            name: "trafficscotland",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "a76[\\D$]|irvine|kilmarnock|a77[\\D$]|m77[\\D$]|bellfield|galston",
            excludes : "safety|careful",
            active_hours : [
                (
                    start: 6,
                    end: 10,
                    excludes: "southbound|s/b",
                ),
                (
                    start: 14,
                    end: 18,
                    excludes:"northbound|s/b",
                )
            ],
            active_days : "Mon|Tue|Wed|Thu|Fri"
        ),
        ( 
            name: "trafficscotland",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "a76[\\D$]|irvine|kilmarnock|a77[\\D$]|m77[\\D$]|bellfield|galston",
            excludes : "safety|careful",
            active_hours : [
                (
                    start: 10,
                    end: 19,
                )
            ],
            active_days : "Sat"
        ),

        ( 
            name: "BBCScotlandNews",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "kilmarnock|ayrshire|scotstoun|m77[\\D$]|a76[\\D$]|a77[\\D$]",
        ),
        ( 
            name: "BBCScotWeather",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],

            includes : "kilmarnock|ayrshire",
        ),
        ( 
            name: "AyrshireEPolice",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],

            includes : "kilmarnock|southcraig|northcraig|galston|loudoun|a77[\\D$]|m77[\\D$]|a76[\\D$]|onthank|wardneuk",
        ),
        ( 
            name: "AyrshirePolice",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "kilmarnock|southcraig|northcraig|galston|loudoun|a77[\\D$]|m77[\\D$]|a76",
        ),
        ( 
            name: "EastAyrshire",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "kilmarnock|northcraig|southcraig|galston|loudoun|a77[\\D$]|m77[\\D$]|a76[\\D$]|onthank",
        ),
        ( 
            name: "PHE_uk",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "dashboard has been updated",
        ),
        ( 
            name: "metofficeWScot",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "warning",
        ),
        ( 
            name: "KilmarnockFC",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "\\d\\d'\\||ht|ft",
        ),
        ( 
            name: "CelticFC",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "half time|ht|ft|full time|kick off|ko",
        ),
        ( 
            name: "RangersFC",
            chats : [
                (chat: -123),
                (chat: 1234),
            ],
            includes : "half time|ht|ft|full time|kick off|ko",
        )
    ]
)
```

# Other notes

- The twitter account being used for the bot, has to follow the accounts mentioned in the rules.
- For now it just lists raw telegram chat IDs. These can be obtained from bots like @myID on telegram
- Text strings are lower cased before matching against includes regex so best not to have upper cases in rules
