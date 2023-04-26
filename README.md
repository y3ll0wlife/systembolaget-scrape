# Systembolaget product scraper

> Written to figure out which product has the best APK (Alcohol per krona)

A scraper for the ~~un~~offical [systembolaget.se](https://www.systembolaget.se/) API.  

The scraper fetches all of the pages of the API and then stores the information in a SQLite database, for easy use later on. It also does provide some prebuilt features for easy to use later on:

- Calculates the [Alcohol per krona (APK) (wikipedia article in Swedish)](https://sv.wikipedia.org/wiki/Alkohol_per_krona)
- Resolves the image and gives you back a correct url
- Gives the url to the product page  

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
