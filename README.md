# Stride stTia airdrop modeling

This is based on the stride airdrop for those who hold stTia according to these details: https://www.stride.zone/blog/airdrop-to-sttia-holders

The assumptions used are the following:
1. Stride trades at $3 in 180+ days from now once vesting is done
2. started StTia TVL at 10mil 
3. increasing TVL per day based on milky way's TVL increase (with 3x multipier) based from defillama csv data I got.
   Defillama didn't start tracking until much later after its release and stride/milky way increases will not be similar in my opinion.
   There is already a lot of liquid Tia in milkTia that will be vampire attacked away to Stride + some cosmos folks were waiting for Stride LST so the amount of TVL increase should be much larger.
   This is clear as 24hr post launch already shows how high TVL

Reasoning on some model choices:
- The modeling is only done for 20 days instead of the full major stride bonus of first 60 days because I decided that it is very likely that the first 10-20 days will return the most.
- There will be a larger influx of stTia users once those users unstake their validator staked tia which is 21 day unstake period will put into stride which is an assumption that users may unstake their native staked tia into stride's liquid staking because stride team has discussed how it is a priority of theirs to ensure that stTia holders get the native staking airdrops.  <br />
This is something to be taken as true because MilkTia holders did qualify for latest tia airdrop, AltLayer:
https://blog.altlayer.io/altlayer-airdrop-season-one-celestia-stakers-registration-phase-now-open-db613cf38351
That coupled with normal TVL increase post-launch tells me the daily return drops off significantly after a few days
- Stride is assumed to trade at $3. This is done to not try to model off bullish or bearish estimates as either can be seen as naive given that it's presumptious to assume that a protocol's token grows a significant amount or dumps a significant amount due to market dynamics.

Results:
The model shows return of 10%+ APR on stTia held for 20 days. I think this model is wrong although average daily returns will likely be in the 1%+ APR.  <br />
![image](https://github.com/meekteek/strd_modeling/assets/95730625/be457f17-2cfe-4f66-b21c-b6526bb5dc8b)

1. Stride is much more likely to be trading closer to $10 than $3 in 180-200 days. Stride has cemented itself as the center of liquid staking on Cosmos as an IBC index aligned protocol who is able to become aware of current IBC related ideas, ship, and iterate faster than any other competitor in the space. The attention and value provided will continue to grow and reprice protocols and defi primitives such as this.
2. TVL increase dataset is inaccurate because dataset that doesn't represent stride's. It's very hard to forecast effects of liquidity being vampire attacked away from milkTia coupled with the participants, groups, or funds who haven't explicitly stated but have been keenly waiting for stride to release stTia since their proposal (https://forum.celestia.org/t/moving-toward-safer-and-more-aligned-tia-liquid-staking/1422) to push size into the protocol. It is and was in their best interests to not speak up unless they managed to get into the latest 4m seed round announced early Feburary (since this readme was updated)

   
When running this data yourself, you must have input.txt in /data folder with number of tia you choose to stake in stTia  <br />
![image](https://github.com/meekteek/strd_modeling/assets/95730625/b78101e3-86b5-4a10-82b5-e58f3adde906)
