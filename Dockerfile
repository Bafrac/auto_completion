FROM python:3.12-alpine 
#Use of alpine for light weight

WORKDIR /app

COPY . .

EXPOSE 8000

CMD ["python", "main.py"]