import requests
import json

url = "http://localhost:8080/reply"
headers = {"Content-Type": "application/json"}
data = {
    "messages": [
        {"role": "user", "content": [{"type": "text", "text": "Say hello"}]}
    ]
}

response = requests.post(url, headers=headers, json=data, stream=True)
print(f"Status: {response.status_code}")
print(f"Headers: {response.headers}")
print("\nResponse stream:")

for line in response.iter_lines():
    if line:
        decoded = line.decode('utf-8')
        print(decoded)
        if decoded.startswith("data: "):
            try:
                event_data = json.loads(decoded[6:])
                print(f"  Parsed: {event_data}")
            except:
                pass
