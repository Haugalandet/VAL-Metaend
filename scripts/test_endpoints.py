import requests

URL = "http://localhost:6969"

GET_POLLS_BY_TITLE = f"{URL}/polls"
GET_ALL_POLLS = f"{URL}/polls/all"

def test_endpoint(url, expected_status):
    response = requests.get(url)
    assert response.status_code == expected_status, f"Test failed for {url}: Expected {expected_status}, got {response.status_code}"

try:
    test_endpoint(f"{GET_POLLS_BY_TITLE}", 404)
    test_endpoint(f"{GET_POLLS_BY_TITLE}/{'TITLE'}", 404)
    test_endpoint(f"{GET_POLLS_BY_TITLE}/?title={'TITLE'}", 200)
    test_endpoint(GET_ALL_POLLS, 200)
    
    # Add more tests as needed
    print("All tests passed!")
except AssertionError as e:
    print(e)
    exit(1)