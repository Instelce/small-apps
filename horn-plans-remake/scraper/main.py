import requests
from bs4 import BeautifulSoup
import json
from lxml import etree

main_url = "http://hornplans.free.fr/"
plan_url = "http://hornplans.free.fr/plan_site.html"

exclude_images = [
    "logo2a.gif",
    "wpimages/wpc863eea5_41.jpg"
    "wpimages/wp90b794cf_41.jpg",
    "wpimages/wpdc0e4921_41.jpg",
    "wpimages/wp6e810c58_41.jpg",
    "linkpartn1.gif",
    "linkpartn1.gif",
    "linkpartn1.gif",
]

data = []
all_soups = []
all_links = []
response = requests.get(plan_url)
plan_soup = BeautifulSoup(response.content, "html.parser")

links = plan_soup.find_all("a")

for i, a in enumerate(links):
    href = a["href"]
    if "html" in href:
        slug = href.split(".")[0]
        print("\nAdd:", slug)

        r = requests.get(main_url + href)
        soup = BeautifulSoup(r.content, "html.parser")
        de = soup.find_all('span', {'class': 'Normal-C'})
        print(de)

        # /html/body/div[40]/div[15]/div[3]/span

        # description = soup.xpath("/html/body/div[40]/div[15]/div[3]/span")[0].text
        description = ""
        if len(de) >= 6:
            description = de[5].text

        pdf = []
        iframes_el = soup.find_all("iframe")
        for iframe in iframes_el:
            pdf.append(main_url + iframe["src"])

        print("PDF:", len(pdf))

        images = []
        images_el = soup.find_all("img")
        for image in images_el:
            src = image.attrs["src"]
            if src not in exclude_images:
                images.append(main_url + src)

        print("Images:", len(images))

        data.append(
            {
                "slug": slug,
                "description": description,
                "page": href,
                "pdf": pdf,
                "images": images,
            }
        )

        all_links.append(href)
        all_soups.append(soup)

        print(i, "/", len(links))

with open("data.json", "w+") as file:
    file.write(json.dumps(data))
