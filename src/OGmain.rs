use regex::Regex;



fn main() {
    let re = Regex::new("revenue").unwrap();

    let quote = " Now moving on to the financials. In my remarks,
    I am going to provide some additional detail on revenue,
    walk through the fourth quarter income statement,
    touch on a few other key financial metrics,
    and then Iâll finish with our financial guidance for 2019. Unless otherwise noted,
    my remarks will focus on non-GAAP results and percentage changes will be on a year-over-year basis.
    As Mike mentioned, we delivered a very strong fourth quarter to finish an excellent fiscal year. 
    Revenue for the quarter was $1.29 billion with core revenue growth of 9% exceeding both our guidance and expectations. 
    For the full fiscal year, our core revenue growth was 7.1%, a very strong performance.
    As Mike spoke to the Groupâs performance for the quarter, 
    I will provide some additional details around our end-market and regional performance. 
    Overall, the market environment is positive and based on our channel reach and product offerings,
    we saw broad strength across most end-markets. Pharma, our largest end-market was up 14% with double-digit contribution from all business groups.
    Both the small molecule and biopharma segments performed well. Traditional areas of strength, as well as newer areas of strategic focus such as cell analysis 
    and a strong performance at NASD contributed to the results. Chemical and energy grew an impressive 7% against a very strong comparison of 15% core growth last year.
    We continue to see positive ongoing market investment in this area. 
    Balanced gains in both LSAG and ACG were driven by strength in spectroscopy, LC-MS, supplies and services.
    Demand for our materials characterization applications continue to drive robust ICP-MS growth.
    Environmental and forensics grew 17% ahead of expectations with good demand across major regions.
    Growth was balanced across both end-markets. Forensic saw notable demand for Cobalt Raman spectroscopy and environmental for LC-MS and ICP-MS.
    Academia and Government reported 10% growth as funding environment stabilized, while diagnostics and clinical grew 1% and food was flat as expected against a tough 10% comparison.
    Geographically, we also saw broad based strength. China grew by 16% accelerating from the 10% core growth we saw in Q3 and as Mike mentioned,
    passed the $1 billion mark in sales for the year in the fourth quarter. Other Asia and Japan grew by 12% and Europe and the Americas had solid mid-single-digit growth. In addition,
    we continue to be pleased with the revenue contribution as non-instrument revenue contributed 56% of the total in Q4.";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}