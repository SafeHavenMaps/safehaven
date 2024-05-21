interface GeocodeAddress {
  county: string
  city: string
  city_district: string
  construction: string
  continent: string
  country: string
  country_code: string
  house_number: string
  neighbourhood: string
  postcode: string
  public_building: string
  state: string
  suburb: string
}

interface NominatimResponse {
  address: GeocodeAddress
  boundingbox: string[]
  class: string
  display_name: string
  importance: number
  lat: string
  licence: string
  lon: string
  osm_id: string
  osm_type: string
  place_id: string
  svg: string
  type: string
}

export interface Result {
  id: string
  title: string
  subtitle: string
  lat: number
  lon: number
}

function fullTextToTitles(fullText: string): { title: string, subtitle: string } {
  const splitted = fullText.split(', ')

  if (splitted.length === 1) {
    // If there is only one element, it's both the title and the subtitle
    return {
      title: splitted[0],
      subtitle: '',
    }
  }
  else if (splitted.length === 2) {
    // If there are two elements, the first is the title, the second is the subtitle
    return {
      title: splitted[0],
      subtitle: splitted[1],
    }
  }
  else if (splitted.length > 2) {
    // If there are more than two elements, the first two elements form the title, the next two form the subtitle
    return {
      title: `${splitted[0]}, ${splitted[1]}`,
      subtitle: `${splitted[2]}, ${splitted[3] || ''}`,
    }
  }
  else {
    throw new Error('Unexpected full text format')
  }
}

export async function freeFormSearch(query: string, take: number = 5): Promise<Result[]> {
  const queryEncoded = encodeURIComponent(query)
  const request = await fetch(`https://nominatim.openstreetmap.org/search?format=json&q=${queryEncoded}`)
  const response: NominatimResponse[] = await request.json()

  return response
    // Sort by importance (the higher the number, the more important the result is)
    .sort((a, b) => b.importance - a.importance)
    // Map the results to a more simple object for the frontend
    .map((result: NominatimResponse) => {
      const titles = fullTextToTitles(result.display_name)
      return {
        ...titles,
        id: result.osm_id,
        lat: parseFloat(result.lat),
        lon: parseFloat(result.lon),
      }
    })
    // deduplicate results using titles and subtitles as reference
    .filter((result, index, self) => self.findIndex(r => r.title === result.title && r.subtitle === result.subtitle) === index)
    // Limit the number of results
    .slice(0, take)
}
