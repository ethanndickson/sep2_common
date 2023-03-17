# Common Resources

These files relate to resources under section B.2.3 of the official 2030.5 specification (page 154).

## Identification

Defines traits and structs described generally in section 8.2 and in detail in section B.2.3.2. specifically, describes the Resource trait and traits that extend it (e.g. List, IdentifiedObject)

For List objects, all is the total number of entries, and result is the number of results returned to the client.
Link objects simply return the URI (as a String) of the Resource they link to. ListLink's return the URI's of List Resources.

## Objects

Described mainly in section 10.2.3 detail in section B.2.3.3. Specifically, it contains the traits and objects for Events and related types that extend it (e.g. RandomizableEvent)

## Types

Described in detail in section B.2.3.4. Specifically, it contains type aliases for data types used for various function sets (e.g. TimeType is a 64-bit integer or "Int64")

## Primitives

Described in detail in section B.2.3.5. Specifically, it contains type aliases primitive, concrete data types (e.g. Int64 is the type i64 in rust)
