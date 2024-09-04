FROM python:alpine

WORKDIR /usr/src/app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY app.py app.py

COPY ./data ./data

CMD [ "gunicorn", "-w 16", "-b 0.0.0.0:8000", "app:app" ]

