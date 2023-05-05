

dotnet aspnet-codegenerator controller \
-name PriceController \
-m Price \
-dc PriceDataContext \
-async \
-api \
-outDir Controllers



dotnet tool install -g Microsoft.dotnet-openapi


dotnet openapi add file scripts/webflux_customer_service.json


nswag openapi2csclient /input:MyWebService.json
        /classname:MyServiceClient
        /namespace:MyNamespace
        /output:MyServiceClient.cs





  AuthenticationService.csproj: [NU1605] Warning As Error: Detected package downgrade: Newtonsoft.Json from 13.0.1 to 12.0.2. Reference the package directly from the project to select a different version. 
 AuthenticationService -> Microsoft.VisualStudio.Web.CodeGeneration.Design 8.0.0-preview.3.23206.5 -> NuGet.Packaging 6.3.1 -> Newtonsoft.Json (>= 13.0.1) 
 AuthenticationService -> Newtonsoft.Json (>= 12.0.2)
 