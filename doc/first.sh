#Navigate to Cargo
cd ../src/cli/
# CLI-Command
cargo run -- --cli  \
--first-name Ferdinand --last-name Strobel \
--phone-number "+49 176 83220561" --email-address ferdistr08@gmail.com  --website https://ferdistro.github.io \
--job-experience "Meine Leidenschaft für Softwareentwicklung zeigte sich früh in eigenen Projekten, wie der Entwicklung einer  eigenen <a href='https://github.com/FerdiStro/Magic-Dungeons'> 2D-Engine</a>.<br>
                      2024 schloss ich meine Ausbildung zum Fachinformatiker für Anwendungsentwicklung bei <a href='https://www.infront.co'>Infront</a> erfolgreich ab und arbeitete dort weiterhin bis September 2025 als Softwareentwickler.<br>
                      Dabei lag mein Schwerpunkt auf Microservice-Architekturen und Cloud-Infrastrukturen mit Java und Spring Boot. Zudem nutzte ich Frontend-Technologien (Angular, Svelte, React Native), etablierte DevOps-Tools (CI/CD, Docker, Kubernetes) sowie Datenbanken (PostgreSQL, Redis).<br>
                      Seit Oktober 2025 entwickle ich bei <a href='https://www.open200.com/'>open200</a> in Wien Individualsoftware.<br>
                      Mein Tech-Stack umfasst hier unter anderm Kotlin, Spring, AxonIQ und React. Ebenfalls konnte ich in dieser Rolle meine DevOps-Expertise (Kubernetes, Cluster-Management) und mein Wissen rund um Keycloak gezielt vertiefen.<br>
                      Zudem sammelte ich im Rahmen der <a href='https://www.linkedin.com/posts/open200_innovation-days-activity-7441777011175366657-5N94?utm_source=share&utm_medium=member_desktop&rcm=ACoAADge5U0BV22RMzQ_nBrLGh-Zb-3OVo65C_c'>Innovation Days</a> praktische Erfahrungen im Agentic Engineering.<br>
                      Agile Tools (Jira, Confluence), Open-Source-Lösungen (Grafana, Rocket.Chat) und Qualitätssicherung (JUnit, SonarQube) gehören zu meinem Standard-Repertoire.<br>
                      In meiner Freizeit verfolge ich weiterhin eigene Projekte rund um <a href='https://github.com/FerdiStro/VS'>Rust</a>, <a href='https://github.com/FerdiStro/CDJ-BeatBoxer'>Embedded systems</a>, <a href='https://github.com/FerdiStro/Jfrog-WAR-deploy'>Cloud-Deployment</a> und <a href='https://github.com/FerdiStro/Tree.js-Playground'>3D-Rendering</a>.<br>" \
--about-me "Ich bin 21 Jahre alt und lebe in Wien.<br>
               In meiner Freizeit treibe ich gerne Sport (Boxen, Bouldern), engagiere mich in der Jugendarbeit und starte
               regelmäßig <br> eigene Softwareprojekte (<a href='https://github.com/FerdiStro'>GitHub</a>).<br>
               Besonders gern kombiniere ich mein technisches Know-how mit meinem DJ-Hobby und meiner Begeisterung für
               Videospiele, um kreative Ideen umzusetzen.<br>" \
--skill "skill_name=Java,rating=5" \
--skill "skill_name=Rust,rating=3" \
--skill "skill_name=Type-Script,rating=4" \
--skill "skill_name=Java-Script,rating=4" --skill "skill_name=Gradle,rating=4" \
--skill "skill_name=Html/CSS,rating=5" --skill "skill_name=TailwindCss,rating=5" \
--skill "skill_name=Spring,rating=5" --skill "skill_name=Springboot,rating=5" \
--skill "skill_name=Angular,rating=5" --skill "skill_name=React-Native,rating=3" \
--skill "skill_name=React,rating=4" \
--language "skill_name=German,rating=5" --language "skill_name=English,rating=5" --language "skill_name=Spanish,rating=1" \
--time-point "type=0,title=Würzburg,description=Geboren am,date=28.03.2005,location=Würzburg,space=false" \
--time-point "type=1,title=Leopold Sonnemann Realschule,description=Abschluss in Mathe und Physik,date=2021,location=Würzburg,space=true" \
--time-point "type=2,title=Infront Financial Technology GmbH,description= Ausbildung Fachinformatiker - Anwendungsentwicklung,date=01.01.2021 - 24.06.2024,location=Würzburg,space=false" \
--time-point "type=2,title=Infront Financial Technology GmbH,description=Full-stack Developer. Mit Fokus auf Spring cloud Entwicklung,date=02.06.2024 - 01.10.2025,location=Frankfurt,space=true" \
--time-point "type=0,title=Wien,description=Umzug nach Wien,date=01.10.2025,location=Wien,space=false" \
--time-point "type=2,title=open200 GmbH,description=Intermediate Softwareentwickler,date=06.10.2025,location=Wien,space=false" \
--time-point "type=2,title=nexxar GmbH,description=Backend Developer (Java) ,date=15.05.26,location=Wien,space=false" \
--job "Backend Developer (Java)" \
--cover "Sehr geehrtes nexxar-Team,<br><br>ich bewerbe mich als Backend Developer. Mit 4,5 Jahren professioneller Erfahrung in der Entwicklung von Microservice-Architekturen möchte ich meine Kenntnisse in Ihre Content-Plattform einbringen.<br><br>Der Grund für meine Bewerbung ist die aktuell unsichere Auftragslage meines derzeitigen Arbeitgebers. Obwohl ich mich dort fachlich und im Team sehr wohl fühle, möchte ich mich frühzeitig neu orientieren und sehe in nexxar eine hervorragende Möglichkeit, mich fachlich und persönlich weiterzuentwickeln.<br><br>Zu Ihrer Frage bezüglich des Java-Snippets: Der Code erscheint technisch korrekt, wobei die Eignung stark vom Kontext abhängt – insbesondere davon, ob Thread-Sicherheit (Atomarität) erforderlich ist oder ob die Ausführung im Single-Thread erfolgt. Den Screenshot habe ich mithilfe von Google Gemini in kopierbaren Text umgewandelt, um den Code in meiner IDE effizienter analysieren zu können.<br><br>Mein Profil passt gut zu Ihrem Stack: Ich arbeite täglich mit Java 11+, Maven und CI/CD-Pipelines via GitLab. Zudem besitze ich Kenntnisse in JavaScript/TypeScript sowie fundierte Erfahrung mit REST-Schnittstellen, gRPC und SOAP. Details zu meinem technischen Hintergrund entnehmen Sie bitte meinem Lebenslauf auf der nächsten Seite. Ich bin ab dem 15.05. verfügbar." \
--color '#66CD13' \
--lang de \
--cover-merged
