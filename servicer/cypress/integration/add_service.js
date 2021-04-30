/* global describe it Blob before */

const REGISTRY_URL = 'http://127.0.0.1:3030'

/* eslint-env mocha */
describe('Service Registry API', () => {

  const initialItems = [
    {
      "name": "Auth Service - Live",
      "url": "http://192.168.15.2",
      "endpoints": ["/auth"],
      "authorized_roles": ["admin", "staff", "customer"]
    },
    {
      "name": "Auth Service - Test",
      "url": "http://192.168.15.3",
      "endpoints": ["/auth"],
      "authorized_roles": ["admin", "staff", "customer"]
    }
  ]

  /* const getItems = () =>
    cy.request('/services')
      .its('body') */


  const add = item =>
    cy.request('POST', 'http://127.0.0.1:3030/v1/services', item)

  /* const deleteItem = item =>
    cy.request('DELETE', `/services/${item.id}`)

  const deleteAll = () =>
    getItems()
      .each(deleteItem) */

  const reset = () => {
    // deleteAll()
    // initialItems.forEach(add)
  }

  beforeEach(reset)
  afterEach(reset)

  /* it('returns JSON', () => {
    cy.request('/services')
      .its('headers')
      .its('content-type')
      .should('include', 'application/json')
  })

  it('loads 2 items', () => {
    cy.request('/services')
      .its('body')
      .should('have.length', 2)
  })

  it('loads the initial items', () => {
    getItems()
      .should('deep.eq', initialItems)
  })

  it('returns id + task objects', () => {
    getItems()
      .each(value =>
        expect(value).to.have.all.keys('id', 'task')
      )
  }) */

  it('Add a new service to registry', () => {
    const item = {
      "name": "Authorize.net Direct payment - Test",
      "url": "http://192.168.15.4",
      "endpoints": ["/authorizenet/direct"],
      "authorized_roles": ["admin", "staff", "customer"]
    }

    cy.request('POST', `${REGISTRY_URL}/v1/services`, item)
      .as('response')
      cy.get('@response').should((response) => {
        expect(response.body).to.be.equal('Added items to the service list')
        expect(response.status).to.be.equal(201)
      })
  })

  /* it('deletes an item', () => {
    const id = initialItems[0].id
    cy.request('DELETE', `/services/${id}`)
    getItems()
      .should('have.length', 1)
  })*/
})