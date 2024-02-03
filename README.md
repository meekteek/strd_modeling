# Stride stTia airdrop modeling

This is based on the stride airdrop for those who hold stTia according to these details: https://www.stride.zone/blog/airdrop-to-sttia-holders


The assumptions used are the following:
1. Stride trades at $3 in 180+ days from now once vesting is done
2. started StTia TVL at 10mil 
3. increasing TVL per day based on milky way's TVL increase (with 3x multipier) based from defillama csv data i got.
   Defillama didn't start tracking until much later after its release and stride/milky way increases will not be similar in my opinion.
   There is already a lot of liquid Tia in milkTia that will be vampire attacked away to Stride + some cosmos folks were waiting for Stride LST so the amount of TVL increase should be much larger.
   This is clear as 24hr post launch already shows how high TVL

The modeling is only done for 20 days instead of the full major stride bonus of first 60 days because I decided that it is very likely that the first 10-20 days will return the most.
There will be a larger influx of stTia users once those users unstake their validator staked tia which is 21 day unstake period will put into stride. 
There is an assumption that users may unstake their native staked tia into stride's liquid staking because stride team has discussed how it is a priority of theirs to ensure that stTia holders get the native staking airdrops. 
This is something others will believe to be true because MilkTia holders did qualify for latest tia airdrop, AltLayer:
https://blog.altlayer.io/altlayer-airdrop-season-one-celestia-stakers-registration-phase-now-open-db613cf38351

That coupled with normal TVL increase post-launch tells me the daily return drops off significantly after a few days

Note: Must have input.txt in /data folder with number of tia you choose to stake in stTia
![image](https://github.com/meekteek/strd_modeling/assets/95730625/b78101e3-86b5-4a10-82b5-e58f3adde906)
