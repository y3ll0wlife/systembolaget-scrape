# Systembolaget product scraper

> Written to figure out which product has the best APK (Alcohol per krona)

A scraper for the ~~un~~offical [systembolaget.se](https://www.systembolaget.se/) API.  

The scraper fetches all of the pages of the API and then stores the information in a SQLite database, for easy use later on. It also does provide some prebuilt features for easy to use later on:

- Calculates the [Alcohol per krona (APK) (wikipedia article in Swedish)](https://sv.wikipedia.org/wiki/Alkohol_per_krona)
- Resolves the image and gives you back a correct url
- Gives the url to the product page  

---

## Top 10 APK (as of writting this)

| #  	| Product Name       	| Category   	| APK (ml/kr) 	| Alcohol Percentage 	| Price (kr) 	| Volume (ml) 	| Link                                              	|
|----	|--------------------	|------------	|-------------	|--------------------	|------------	|-------------	|---------------------------------------------------	|
| 1  	| Pascal Costeau     	| Rött vin   	| 2.4         	| 13.5               	| 169.0      	| 3000.0      	| https://www.systembolaget.se/produkt/vin/5287308/ 	|
| 2  	| Arboga 10,2        	| Ljus lager 	| 2.33        	| 10.2               	| 21.9       	| 500.0       	| https://www.systembolaget.se/produkt/ol/1139212/  	|
| 3  	| Sofiero Original   	| Ljus lager 	| 2.29        	| 7.5                	| 16.4       	| 500.0       	| https://www.systembolaget.se/produkt/ol/127312/   	|
| 4  	| Sju komma tvåan    	| Ljus lager 	| 2.26        	| 7.2                	| 15.9       	| 500.0       	| https://www.systembolaget.se/produkt/ol/156812/   	|
| 5  	| Three Hearts       	| Ljus lager 	| 2.22        	| 7.5                	| 16.9       	| 500.0       	| https://www.systembolaget.se/produkt/ol/1122812/  	|
| 6  	| Pripps Extra Stark 	| Ljus lager 	| 2.2         	| 7.2                	| 16.4       	| 500.0       	| https://www.systembolaget.se/produkt/ol/129612/   	|
| 7  	| Outlaw             	| Rött vin   	| 2.19        	| 14.5               	| 199.0      	| 3000.0      	| https://www.systembolaget.se/produkt/vin/5958508/ 	|
| 8  	| Il Capolavoro      	| Rött vin   	| 2.19        	| 14.5               	| 199.0      	| 3000.0      	| https://www.systembolaget.se/produkt/vin/5328508/ 	|
| 9  	| Castillo de Gredos 	| Rött vin   	| 2.18        	| 13.0               	| 179.0      	| 3000.0      	| https://www.systembolaget.se/produkt/vin/1279708/ 	|
| 10 	| Böda               	| Rosévin    	| 2.14        	| 11.0               	| 154.0      	| 3000.0      	| https://www.systembolaget.se/produkt/vin/5202408/ 	|

---

## How to use?

Fill out the `.env.example` and rename to `.env`  
The value `SYSTEMBOLAGET_API_KEY` is your user api key on the website, in the headers look for `Ocp-Apim-Subscription-Key` and copy the value. This shows up on the search requests and seems to never expire.

Then its just runnign it, as of now there is no prebuilt binaries but download [Rust](https://www.rust-lang.org/) and then just run it with `cargo run`  

---

## (Fun?) Observations from the website

Systembolaget will tell you they have around 24,000 products in store, but when looking through the results from the API I am only able to find around 9,000 unique products (basing this on the `product_id` field). A possible explanation for this could be that only 9,000 of the products are currently in the stores and but during all of its existince there has been 24,000 products.

The urls don't need to be one a spefic way. For example this: `https://www.systembolaget.se/produkt/this_can_be_whatever/just_include_the_product_number_and_it_works-269912/` will work the same as `https://www.systembolaget.se/produkt/vin/santa-helena-269912/`.

As said before the `Ocp-Apim-Subscription-Key` values never seem to expire, so we can use them forever and ever.
