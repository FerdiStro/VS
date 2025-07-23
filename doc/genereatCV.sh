#Navigate to Cargo
cd ../cli/grrs/
# CLI-Command
cargo run -- --cli  \
--first-name FirstName --last-name LastName \
--phone-number "+49 001 23455540" --email-address yourEmail@gmail.com \
--skill "skill_name=Java,rating=5" --skill "skill_name=C,rating=2" \
--skill "skill_name=Python,rating=3" --skill "skill_name=Rust,rating=3" \
--skill "skill_name=C++,rating=2" --skill "skill_name=Type-Script,rating=4" \
--skill "skill_name=Java-Script,rating=4" --skill "skill_name=Gradle,rating=4" \
--skill "skill_name=Html/CSS,rating=5" --skill "skill_name=TailwindCss,rating=5" \
--skill "skill_name=Spring,rating=5" --skill "skill_name=Springboot,rating=5" \
--skill "skill_name=Angular,rating=5" --skill "skill_name=React-Native,rating=5" \
--skill "skill_name=Linux,rating=5" --skill "skill_name=Docker,rating=5" \
--skill "skill_name=Docker-Compose,rating=5" --skill "skill_name=Kubernetes,rating=4" \
--skill "skill_name=JFrog,rating=3" --skill "skill_name=Bitbucket-pipeline,rating=5" \
--skill "skill_name=Github Actions,rating=5" \
--language "skill_name=German,rating=5" --language "skill_name=English,rating=5" --language "skill_name=Spanish,rating=1" \
--time-point "type=0,title=CityX,description=Born on,date=28.03.2005,location=CityX,space=true" \
--time-point "type=1,title=SCHOOL-X,description=Degree in Mathematics and Physics,date=2000,location=CityX,space=true" \
--time-point "type=2,title=It Company Software GmbH (application developer),date=01.01.2020 - 01.01.2025,location=CityX,space=false" \
--time-point "type=2,title=Infront Financial Technology GmbH,description=Full-stack developer. Main focus on Spring cloud development,date=02.06.2024 - 01.10.2025,location=Frankfurt,space=true" \
--time-point "type=0,title=CityY,description=Move to CityY,date=01.05.2025,location=CityY,space=true" \
--time-point "type=2,title=Future Company GmbH,description=Senior Software Engineer ,date=01.05.2025,location=CityY,space=false" \





