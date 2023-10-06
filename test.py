import requests


def test():
    url = 'http://127.0.0.1:8000/'
    res = requests.post(url, json={
        "url": "https://tuna2134.jp/"
    })
    print(res.text)


if __name__ == '__main__':
    test()