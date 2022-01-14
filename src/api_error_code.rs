use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum ApiErrorCode {
    Unknown = 0,

    // System exception
    // Something went wrong on our end, please contact us.
    SystemException = 1000003,

    // Invalid response type
    // The provided response type(Accept) was invalid.
    InvalidResponseType = 1000030,

    // Invalid content type
    // The provided content type was invalid.
    InvalidContentType = 1000031,

    // Värdet måste vara alfanumeriskt ({Value})
    // The value needs to be alphanumeric.
    NotAlphanumeric = 2000106,

    // Värdet måste vara numeriskt ({Value})
    // The value needs to be numeric.
    NotNumeric = 2000108,

    // Värdet måste vara en boolean ({Value})
    // The value needs to be boolean.
    NotBoolean = 2000134,

    // Ogiltig inloggning
    // The Client-Secret or the Access-Token is either missing or is incorrect.
    InvalidCredentials = 2000310,

    // Kan inte logga in, access-token eller client-secret saknas(2).
    // The Client-Secret or the Access-Token is either missing or is incorrect.
    InvalidCredentials2 = 2000311,

    // Värdet innehåller ej tillåtna tecken. ({Value})
    // The value contains invalid characters.
    InvalidCharacters = 2000359,

    // Ogiltig parameter i anropet.
    // A parameter is invalid. Read more about parameters.
    InvalidParameters = 2000588,

    // Kundnummer 1 används redan. Kundnumret har redan använts men blivit raderat.
    // Customer number 1 is/has already been used.
    CustomerNumberHasAlreadyBeenUsed = 2000637,

    // A valid identifier was not provided.
    // A valid identifier was not provided.
    AValidIdentifierWasNotProvided = 2000729,

    // Api-licens saknas.
    // The requested Fortnox account does not have a license to use the API.
    NoLicense = 2001103,

    // Ingen eller felaktig typ av data.
    // The request body was empty or contained incorrect data.
    IncorrectData = 2001392,

    // Inläsning av dokument misslyckades: {Message}
    // The XML object contained an error.
    CouldNotReadDocument = 2001740,

    // Error deserializing JSON: JSON_ERROR_SYNTAX
    // The JSON object contained an error.
    CouldNotDeserializeJson = 2002115,

    // Kunde inte hitta konto
    // Could not find Account.
    AccountNotFound = 2001304,

    // Felaktigt fältnamn
    // Invalid Field name.
    InvalidFieldName = 2001399,

    // Det finns ingen aktiv licens för önskat scope
    // There is no active licens for the desired scope.
    NoActiveLicenseForScope = 2001101,

    // Har inte behörighet för scope
    // No access to the current scope.
    NoAccessToScope = 2000663,

    // Det saknas ett förvalt konto för Inköp SE, omvänd skattskyldighet
    // Account is missing for Purchase SE reversed tax liability.
    AccountIsMissingForPurchaseSeReversedTaxLiability = 2003095,

    // Leverantörsfakturan balanserar inte
    // Supplier invoice does not balance.
    SupplierInvoiceDoesNotBalance = 2000755,

    // Momsrader för momstyp REVERSE måste vara märkta med motsvarande CODE
    // Tax Rows for VAT type REVERSE must be marked with CODE.
    TaxRowsForVatTypeReverseMustBeMarkedWithCode = 2003115,

    // Enbart ordrar som levererats ut kan klarmarkeras
    OnlyDeliveredOrdersCanBeMarkedCompleted = 2003124,

    // Ett klarmarkerat dokument kan inte ändras
    CannotChangeCompletedDocument = 2003125,

    // Utleveransdatum kan inte vara senare än dagens datum
    DeliveryDateCannotBeLaterThanToday = 2003126,

    // Migrering är redan påbörjad eller avslutad
    MigrationAlreadyStartedOrCompleted = 2003241,

    // Ej autentiserad
    NotAuthenticated = 2003275,

    // Hittades inte i lagermodulen
    CouldNotBeFoundInWarehouseModule = 2003277,

    // Dokumentet är makulerat i lagermodulen
    DocumentIsDeletedInWarehouseModule = 2003399,

    // Ett fel uppstod i lagermodulen
    AnErrorOccurredInWarehouseModule = 2003127,

    // Kunde inte hämta/hitta kund (kundnummer)
    // The customer in the request is not available in the customer resource.
    CouldNotFindCustomer = 2000204,

    // Kunde inte hämta/hitta kund (kundnummer)
    // The customer in the request is not available in the customer resource.
    CouldNotFindCustomer2 = 2000433,

    // Kunde inte hitta artikel
    // Could not find article used in request.
    CouldNotFindArticle = 2001302,

    // Kan inte hitta artikeln
    CouldNotFindArticle2 = 2000428,
}
