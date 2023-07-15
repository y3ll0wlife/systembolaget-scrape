# Systembolaget product scraper

> Written to figure out which product has the best APK (Alcohol per krona)

A scraper for the ~~un~~offical [systembolaget.se](https://www.systembolaget.se/) API.

The scraper fetches all of the pages of the API and then stores the information in a SQLite database, for easy use later on. It also does provide some prebuilt features for easy to use later on:

- Calculates the [Alcohol per krona (APK) (wikipedia article in Swedish)](https://sv.wikipedia.org/wiki/Alkohol_per_krona)
- Resolves the image and gives you back a correct url
- Gives the url to the product page

---

## Top 10 APK

| APK (ml/kr) | Product Name        | Price (kronor) | Alcohol Percentage | Volume (ml) | Category   | Link                                              |
| :---------- | :------------------ | :------------- | :----------------- | :---------- | :--------- | :------------------------------------------------ |
| 2.42        | Côtes du Roussillon | 60.0           | 14.5               | 1000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7808901/ |
| 2.4         | Pascal Costeau      | 169.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5287308/ |
| 2.4         | Lyngrove            | 175.0          | 14.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/2800808/ |
| 2.4         | Cigarra Gran Passo  | 39.0           | 12.5               | 750.0       | Rosévin    | https://www.systembolaget.se/produkt/vin/256301/  |
| 2.35        | Zumbali             | 179.0          | 14.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5166408/ |
| 2.33        | Arboga 10,2         | 21.9           | 10.2               | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/1139212/  |
| 2.3         | Balestino           | 189.0          | 14.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5104208/ |
| 2.29        | Sofiero Original    | 16.4           | 7.5                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/127312/   |
| 2.26        | Sju komma tvåan     | 15.9           | 7.2                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/156812/   |
| 2.22        | Three Hearts        | 16.9           | 7.5                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/1122812/  |
